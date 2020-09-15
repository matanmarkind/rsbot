use once_cell::sync::Lazy;
use serde::Deserialize;
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::fs::File;
use std::ops::Sub;
use std::sync::RwLock;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    pub filename: String,

    #[structopt(long, parse(try_from_str), default_value = "50000")]
    pub max_no_move_time_us: i64,

    #[structopt(long, parse(try_from_str), default_value = "30")]
    pub max_rows_per_batch: usize,
}
pub static CONFIG: Lazy<RwLock<Option<Config>>> = Lazy::new(|| RwLock::new(None));

#[derive(Debug, Deserialize, PartialEq)]
struct Location {
    time_us: i64,
    x: i32,
    y: i32,
}

const ZERO_LOC: Location = Location {
    time_us: 0,
    x: 0,
    y: 0,
};

// Implementing subtraction by reference to avoid:
// a. consume values on subtraction, which is surprising and annoying.
// b. Automatically copying which is also surprising to user and seems inefficient.
// The downside is that this creates a weird usage syntax (&a - &b).
impl Sub for &Location {
    type Output = Location;

    fn sub(self, other: &Location) -> Location {
        Location {
            time_us: self.time_us - other.time_us,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Ordering and equality is done by the distance only.
#[derive(PartialOrd, Debug)]
struct PathSummary {
    distance: i32,
    avg_time_us: i32,
    time_stdev: i32,
    // Angle of the line from x axis in radians [0, 2PI)
    angle_rads: f32,
}
impl PartialEq for PathSummary {
    fn eq(&self, other: &PathSummary) -> bool {
        self.distance == other.distance
    }
}
impl Eq for PathSummary {}
impl Ord for PathSummary {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}
#[derive(PartialEq, PartialOrd, Debug)]
struct DeltaPosition {
    dx: i32,
    dy: i32,
}
type MousePath = Vec<DeltaPosition>;
type MousePaths = BTreeMap<PathSummary, MousePath>;
fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

// Calculate the angle from the positive x axis to a line pointed from (0, 0) to (dx, dy).
// Results are on the range [0, 2PI)
// Correctly converts dy=0 to an angle of 0.
fn dx_dy_to_angle_rads(dx: f32, dy: f32) -> f32 {
    let slope = (dx / dy).atan();
    if dx >= 0.0 {
        if dy >= 0.0 {
            slope
        } else {
            2.0 * std::f32::consts::PI + slope
        }
    } else {
        std::f32::consts::PI + slope
    }
}

// Takes in a list of changes in the mouse location that correspond to a single path and
// encodes in a way that can be looked up and followed in the future for replay.
// Doesn't check for leading/trailing 0's.
fn parse_mouse_path(delta_mouse_locs: &[Location]) -> (PathSummary, MousePath) {
    let mut path: MousePath = MousePath::new();
    let mut times_us = Vec::<f32>::new();
    let mut total_dx = 0;
    let mut total_dy = 0;
    for Location {
        time_us: dt_us,
        x: dx,
        y: dy,
    } in delta_mouse_locs
    {
        // Iterate through the deltas and parse where movements begin and end.
        times_us.push(*dt_us as f32);
        total_dx += dx;
        total_dy += dy;
        path.push(DeltaPosition { dx: *dx, dy: *dy });
    }

    // Get the net angle of the path drawn by the mouse.
    let angle_rads = dx_dy_to_angle_rads(total_dx as f32, total_dy as f32);

    let summary = PathSummary {
        distance: ((total_dx.pow(2) + total_dy.pow(2)) as f32).sqrt().round() as i32,
        avg_time_us: mean(&times_us[..]).unwrap().round() as i32,
        time_stdev: std_deviation(&times_us[..]).unwrap().round() as i32,
        angle_rads: angle_rads,
    };
    (summary, path)
}

// Take a stream of mouse Locations, and parse them into the actual mouse movements within.
// 'delta_mouse_locs' is expected to be long enough to contain multiple full mouse movements.
fn parse_mouse_deltas(delta_mouse_locs: Vec<Location>) -> MousePaths {
    let max_no_move_time_us = CONFIG.read().unwrap().as_ref().unwrap().max_no_move_time_us;
    if delta_mouse_locs.is_empty() {
        return MousePaths::new();
    }

    let mut mouse_paths: MousePaths = MousePaths::new();
    // Start of the movement. First non 0 Delta in the path.
    let mut path_start_index = 0;
    // Used to ignore trailing 0's when parsing a path.
    let mut last_move_index = 0;
    // If the position doesn't change for long enough, this indicates the end of a movement
    let mut time_since_last_delta_us = 0;
    for (
        i,
        Location {
            time_us: dt_us,
            x: dx,
            y: dy,
        },
    ) in delta_mouse_locs.iter().enumerate()
    {
        if dx == &0 && dy == &0 {
            // Track for how long there has been no movement to determine when the mouse is at rest, and
            // therefore a path is complete. Don't update 'last_move_index' as a way of automatically
            // truncating trailing 0's when the path completes.
            time_since_last_delta_us += dt_us;
            continue;
        }

        if path_start_index == 0 {
            // Special case to truncate leading 0's at the beginning of the batch.
            path_start_index = i;
        }

        if path_start_index < last_move_index
            && (time_since_last_delta_us > max_no_move_time_us || i == delta_mouse_locs.len() - 1)
        {
            // We are now at the end of a single path.
            let (summary, path) =
                parse_mouse_path(&delta_mouse_locs[path_start_index..=last_move_index]);
            mouse_paths.insert(summary, path);
            // Reset the new path beginning.
            path_start_index = i;
        }

        time_since_last_delta_us = 0;
        last_move_index = i;
    }

    match delta_mouse_locs.last() {
        Some(Location {
            time_us: _,
            x: dx,
            y: dy,
        }) => {
            // If the final entry has (dx, dy) == (0, 0) we would have skipped parsing to the end
            // and not parsed the final path. Parse it here.
            if dx == &0 && dy == &0 {
                let (summary, path) =
                    parse_mouse_path(&delta_mouse_locs[path_start_index..=last_move_index]);
                mouse_paths.insert(summary, path);
            }
        }
        _ => println!("unexpected..."),
    }
    println!("{:#?}", mouse_paths);
    mouse_paths
}

// Performs the logic of reading from the CSV and parsing it into a map.
// This is where tests should hook in.
fn parse_csv_input<ReaderT: std::io::Read>(mut reader: csv::Reader<ReaderT>) {
    let max_rows_per_batch = CONFIG.read().unwrap().as_ref().unwrap().max_rows_per_batch;
    // Loop over each record.
    let mut old_mouse_loc = ZERO_LOC;
    let mut delta_mouse_locs = Vec::<Location>::new();
    for (i, result) in reader.deserialize().enumerate() {
        // Create diff
        // Once reach 0,0,0 parse diff.

        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let mouse_loc: Location = result.expect("a Record");

        if old_mouse_loc == ZERO_LOC {
            // This element is the beginning of a new movement. Therefore we can't generate
            // a diff yet.
            old_mouse_loc = mouse_loc;
            continue;
        } else if mouse_loc == ZERO_LOC || delta_mouse_locs.len() >= max_rows_per_batch {
            println!("11111111");
            old_mouse_loc = mouse_loc;
            parse_mouse_deltas(delta_mouse_locs);
            delta_mouse_locs = Vec::<Location>::new();
            continue;
        }

        // Append the change and continue.
        delta_mouse_locs.push(&mouse_loc - &old_mouse_loc);
        old_mouse_loc = mouse_loc;
        if i > 2 * max_rows_per_batch + 1 {
            // println!("{:#?}", delta_mouse_locs);
            println!("break");
            break;
        }
    }
}

// The `main` function is where your program starts executing.
pub fn parse() {
    println!("{:?}", CONFIG);

    // Read the list of timestamps and mouse locations in.
    let file = File::open(&CONFIG.read().unwrap().as_ref().unwrap().filename).unwrap();
    let reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    parse_csv_input::<File>(reader);
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_path() {
        let data = "\
        time_us,x,y
        1,1,1
        2000,1,1
        3000,2,1
        4000,10,20
        5000,13,23
        10000,10,25
        15000,15,31
        ";
        let config = super::Config {
            filename: String::from(""),
            max_no_move_time_us: 10000,
            max_rows_per_batch: 10,
        };
        *super::CONFIG.write().unwrap() = Some(config);
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(data.as_bytes());
        super::parse_csv_input::<&[u8]>(reader);
    }
}
