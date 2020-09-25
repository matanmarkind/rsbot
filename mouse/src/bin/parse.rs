use mouse::constants::*;
use mouse::types::*;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.

    #[structopt(long)]
    pub out_fpath: String, // Serialized output of mouse paths.

    #[structopt(long, parse(try_from_str), default_value = "50000")]
    pub max_no_move_time_us: i64,

    #[structopt(long, parse(try_from_str), default_value = "30")]
    pub max_rows_per_batch: usize,

    // Max number of pixels the mouse can move in a single delta in a given
    // dimension.
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub max_1d_delta: i32,

    // Sanity checks for a path. Max total time a single path can take in
    // seconds.
    #[structopt(long, parse(try_from_str), default_value = "10")]
    pub max_total_path_time_s: i64,

    // Used to only parse part of the CSV. This is useful for testing to shorten
    // time.
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub max_rows_to_read: usize,
}

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn get_net_delta(deltas: &MousePath) -> DeltaPosition {
    deltas
        .iter()
        .fold(DeltaPosition::new(), |cum, delta| &cum + delta)
}

pub struct MousePathParser {
    pub config: Config,
}

impl MousePathParser {
    // Takes in a list of changes in the mouse location that correspond to a single
    // path and encodes in a way that can be looked up and followed in the future
    // for replay. Also performs sanity checks on the path. Doesn't check for
    // leading/trailing 0's.
    fn parse_mouse_path(&self, delta_mouse_locs: &[Location]) -> Option<(PathSummary, MousePath)> {
        let max_total_path_time_us = (1e6 as i64) * self.config.max_total_path_time_s;

        let mut path: MousePath = MousePath::new();
        let mut times_us = Vec::<f32>::new();
        for Location {
            time_us: dt_us,
            x: dx,
            y: dy,
        } in delta_mouse_locs
        {
            times_us.push(*dt_us as f32);
            path.push(DeltaPosition { dx: *dx, dy: *dy });
        }

        if times_us.iter().sum::<f32>().round() as i64 > max_total_path_time_us {
            return None;
        }

        let net_delta = get_net_delta(&path);

        let summary = PathSummary {
            distance: net_delta.distance(),
            avg_time_us: mean(&times_us[..]).unwrap().round() as i32,
            angle_rads: net_delta.angle_rads(),
        };
        Some((summary, path))
    }

    // Take a stream of mouse Locations, and parse them into the actual mouse
    // movements within 'delta_mouse_locs' is expected to be long enough to contain
    // multiple full mouse movements.
    fn parse_mouse_deltas(&self, delta_mouse_locs: Vec<Location>, mouse_paths: &mut MousePaths) {
        if delta_mouse_locs.is_empty() {
            return;
        }

        // Start of the movement. First non 0 Delta in the path.
        let mut path_start_index = 0;
        // Used to ignore trailing 0's when parsing a path.
        let mut last_move_index = 0;
        // If the position doesn't change for long enough, this indicates the end of
        // a movement
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
            if *dx == 0 && *dy == 0 {
                // Track for how long there has been no movement to determine when
                // the mouse is at rest, and therefore a path is complete. Don't
                // update 'last_move_index' as a way of automatically truncating
                // trailing 0's when the path completes.
                time_since_last_delta_us += dt_us;
                continue;
            }

            if path_start_index == 0 {
                // Special case to truncate leading 0's at the beginning of the
                // batch. This can result in us missing the first delta in a batch,
                // but that's not a big deal since mouse paths don't need to be
                // exactly as recorded.
                path_start_index = i;
            }

            if path_start_index < last_move_index
                && (time_since_last_delta_us > self.config.max_no_move_time_us)
            {
                // We are now at the end of a single path.
                match self.parse_mouse_path(&delta_mouse_locs[path_start_index..=last_move_index]) {
                    Some((summary, path)) => mouse_paths.insert(summary, path),
                    None => None,
                };
                // Reset the new path beginning.
                path_start_index = i;
            }

            time_since_last_delta_us = 0;
            last_move_index = i;
        }

        // Parse the path that ends at the end of the batch.
        if path_start_index < last_move_index {
            match self.parse_mouse_path(&delta_mouse_locs[path_start_index..=last_move_index]) {
                Some((summary, path)) => mouse_paths.insert(summary, path),
                None => None,
            };
        }
    }

    // Performs the logic of reading from the CSV and parsing it into a map. This is
    // where tests should hook in.
    pub fn parse_csv_input<ReaderT: std::io::Read>(
        &self,
        mut reader: csv::Reader<ReaderT>,
    ) -> MousePaths {
        let mut mouse_paths = MousePaths::new();

        // Used to print out info to the user.
        let mut max_delta_location = Location {
            x: 0,
            y: 0,
            time_us: 0,
        };
        // Loop over each record to calculate how the mouse moved. Parsing adjacent
        // rows into deltas and groups of rows into mouse paths.
        let mut old_mouse_loc = ZERO_LOC;
        let mut delta_mouse_locs = Vec::<Location>::new();
        for (i, result) in reader.deserialize().enumerate() {
            let mouse_loc: Location = match result {
                Ok(mouse_loc) => mouse_loc,
                _ => {
                    println!("bad line, i={}", i);
                    continue;
                }
            };

            if old_mouse_loc == ZERO_LOC {
                // This element is the beginning of a new movement. Therefore we
                // can't generate a diff yet.
                old_mouse_loc = mouse_loc;
                continue;
            } else if mouse_loc == ZERO_LOC
                || delta_mouse_locs.len() >= self.config.max_rows_per_batch
            {
                old_mouse_loc = mouse_loc;
                self.parse_mouse_deltas(delta_mouse_locs, &mut mouse_paths);
                delta_mouse_locs = Vec::<Location>::new();
                continue;
            }

            let mut delta = &mouse_loc - &old_mouse_loc;
            if (delta.x + delta.y) > (max_delta_location.x + max_delta_location.y) {
                max_delta_location = delta.clone();
            }
            if (delta.time_us as u128) < MIN_TIME_BETWEEN_LOCATIONS.as_micros()
                || (delta.time_us as u128) > MAX_TIME_BETWEEN_LOCATIONS.as_micros()
                || delta.x > self.config.max_1d_delta
                || delta.y > self.config.max_1d_delta
            {
                // Invalid delta, rewrite as a 0 delta.
                delta = ZERO_LOC;
            }
            delta_mouse_locs.push(delta);
            old_mouse_loc = mouse_loc;

            if self.config.max_rows_to_read > 0 && i > self.config.max_rows_to_read {
                println!("break");
                break;
            }
        }
        dbg!(max_delta_location);
        // If we finished the file parse anything left.
        self.parse_mouse_deltas(delta_mouse_locs, &mut mouse_paths);
        mouse_paths
    }

    // The `main` function is where your program starts executing.
    pub fn parse(&self) {
        // Read the list of timestamps and mouse locations in.
        let input_file = File::open(&self.config.in_fpath).unwrap();
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(input_file);

        // Parse the input and convert the record to a list of mouse paths.
        let mouse_paths = self.parse_csv_input::<File>(reader);
        dbg!(mouse_paths.len());

        // Serialize the map that is parsed out from the CSV input and save it to a file.
        let serpaths = bincode::serialize(&mouse_paths).unwrap();
        let mut output_file = File::create(&self.config.out_fpath).unwrap();
        output_file.write_all(&serpaths[..]).unwrap();
    }
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);
    let parser = MousePathParser { config };
    parser.parse();
}

#[cfg(test)]
mod tests {
    // Remember not to have any leading whitespace in rows for the CSV raw string.
    use super::DeltaPosition;
    use structopt::StructOpt;

    // Pass in a string of a CSV of mouse locations. Create a local reader and parse the input.
    fn parse_str(data: &str) -> super::MousePaths {
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(data.as_bytes());

        // Unittests don't read/write from/to files so leave the fields blank. Other fields are left as default values.
        let parser = super::MousePathParser {
            config: super::Config::from_iter(&["", "--in-fpath", "", "--out-fpath", ""]),
        };
        parser.parse_csv_input::<&[u8]>(reader)
    }

    // This function takes in the mouse_paths output from 'parse_csv_input' and
    // checks that it contains the expected path.
    fn check_paths(expected: Vec<(super::MousePath, usize)>, actual: super::MousePaths) {
        assert_eq!(actual.len(), expected.len());

        for (expected_path, total_expected_time) in expected {
            let net_delta_expected = super::get_net_delta(&expected_path);
            let expected_summary = super::PathSummary {
                distance: net_delta_expected.distance(),
                avg_time_us: (total_expected_time / expected_path.len()) as i32,
                angle_rads: net_delta_expected.angle_rads(),
            };
            let (actual_summary, actual_path) = actual.get_key_value(&expected_summary).unwrap();

            assert_eq!(actual_summary.distance, expected_summary.distance);
            assert!((actual_summary.avg_time_us - expected_summary.avg_time_us).abs() <= 1);
            assert!((actual_summary.angle_rads - expected_summary.angle_rads).abs() < 0.01);
            assert_eq!(actual_path, &expected_path);
        }
    }

    #[test]
    fn single_path() {
        let data = "\
time_us,x,y
0,0,0
1000,1,1
11000,2,2
22000,3,1
32300,10,20
42000,13,23
52000,10,25
61500,15,31
";

        let mouse_paths = parse_str(data);

        // The first delta is not (1, 1). This is as a result of skipping 0
        // deltas at the beginning of a batch in 'parse_mouse_deltas'. We don't
        // need to be exact so it's fine and not worth investing in changing.
        let expected_deltas = vec![
            DeltaPosition { dx: 1, dy: -1 },
            DeltaPosition { dx: 7, dy: 19 },
            DeltaPosition { dx: 3, dy: 3 },
            DeltaPosition { dx: -3, dy: 2 },
            DeltaPosition { dx: 5, dy: 6 },
        ];
        check_paths(vec![(expected_deltas, 61500 - 11000)], mouse_paths);
    }

    #[test]
    fn zero_padding() {
        // Check that both leading and trailing 0's are truncated.
        let data = "\
time_us,x,y
0,0,0
0,0,0
0,0,0
1000,1,1
1000,1,1
1000,1,1
1000,1,1
1000,1,1
11000,2,2
22000,3,1
33000,10,20
43000,13,23
53000,10,25
63000,15,31
63000,15,31
63000,15,31
63000,15,31
63000,15,31
63000,15,31
";

        let mouse_paths = parse_str(data);

        // The first delta is not (1, 1). This is as a result of skipping 0
        // deltas at the beginning of a batch in 'parse_mouse_deltas'. We don't
        // need to be exact so it's fine and not worth investing in changing.
        let expected_deltas = vec![
            DeltaPosition { dx: 1, dy: 1 },
            DeltaPosition { dx: 1, dy: -1 },
            DeltaPosition { dx: 7, dy: 19 },
            DeltaPosition { dx: 3, dy: 3 },
            DeltaPosition { dx: -3, dy: 2 },
            DeltaPosition { dx: 5, dy: 6 },
        ];
        check_paths(vec![(expected_deltas, 63000 - 1000)], mouse_paths);
    }

    #[test]
    fn ignore_timeing_error() {
        // Check that if there's an issue with a single recording that takes too
        // A single batch with 2 paths. Indicated by a long pause of 0 deltas.

        let data = "\
time_us,x,y
0,0,0
1000,1,1
11000,2,2
22000,3,1
33000,10,20
44000,13,23
54000,10,25
64000,15,31
193000,20,20
203000,10,20
214000,15,25
223500,25,34
234000,30,34
244000,32,37
254000,37,50
";

        let mouse_paths = parse_str(data);

        // The first delta is not (1, 1). This is as a result of skipping 0
        // deltas at the beginning of a batch in 'parse_mouse_deltas'. We don't
        // need to be exact so it's fine and not worth investing in changing.
        let expected_deltas1 = vec![
            DeltaPosition { dx: 1, dy: -1 },
            DeltaPosition { dx: 7, dy: 19 },
            DeltaPosition { dx: 3, dy: 3 },
            DeltaPosition { dx: -3, dy: 2 },
            DeltaPosition { dx: 5, dy: 6 },
            // Single gap of 50k us gets zeroed out. Cosidered a recording error in the path
            DeltaPosition { dx: 0, dy: 0 },
            DeltaPosition { dx: -10, dy: 0 },
            DeltaPosition { dx: 5, dy: 5 },
            DeltaPosition { dx: 10, dy: 9 },
            DeltaPosition { dx: 5, dy: 0 },
            DeltaPosition { dx: 2, dy: 3 },
            DeltaPosition { dx: 5, dy: 13 },
        ];
        check_paths(
            vec![(expected_deltas1, (64000 - 11000) + (254000 - 193000))],
            mouse_paths,
        );
    }

    #[test]
    fn multiple_paths() {
        // A single batch with 2 paths. Indicated by a long pause of 0 deltas.
        let data = "\
time_us,x,y
0,0,0
1000,1,1
11000,2,2
22000,3,1
33000,10,20
44000,13,23
54000,10,25
64000,15,31
74000,15,31
84000,15,31
94000,15,31
104000,15,31
114000,15,31
124000,15,31
134000,15,31
144000,15,31
154000,15,31
164000,15,31
174000,15,31
184000,15,31
193500,20,20
203000,10,20
214000,15,25
223500,25,34
234000,30,34
244000,32,37
254000,37,50
";

        let mouse_paths = parse_str(data);

        // The first delta is not (1, 1). This is as a result of skipping 0
        // deltas at the beginning of a batch in 'parse_mouse_deltas'. We don't
        // need to be exact so it's fine and not worth investing in changing.
        let expected_deltas1 = vec![
            DeltaPosition { dx: 1, dy: -1 },
            DeltaPosition { dx: 7, dy: 19 },
            DeltaPosition { dx: 3, dy: 3 },
            DeltaPosition { dx: -3, dy: 2 },
            DeltaPosition { dx: 5, dy: 6 },
        ];
        let expected_deltas2 = vec![
            DeltaPosition { dx: 5, dy: -11 },
            DeltaPosition { dx: -10, dy: 0 },
            DeltaPosition { dx: 5, dy: 5 },
            DeltaPosition { dx: 10, dy: 9 },
            DeltaPosition { dx: 5, dy: 0 },
            DeltaPosition { dx: 2, dy: 3 },
            DeltaPosition { dx: 5, dy: 13 },
        ];
        check_paths(
            vec![
                (expected_deltas1, 64000 - 11000),
                (expected_deltas2, 254000 - 184000),
            ],
            mouse_paths,
        );
    }

    #[test]
    fn multiple_batches() {
        // Multiple batches, separated by ZER_LOC.
        let data = "\
time_us,x,y
0,0,0
1000,1,1
11000,2,2
22000,3,1
33000,10,20
44000,13,23
54000,10,25
64000,15,31
0,0,0
184000,15,31
193500,20,20
203000,10,20
214000,15,25
223500,25,34
234000,30,34
244000,32,37
254000,37,50
";

        let mouse_paths = parse_str(data);

        // The first delta is not (1, 1). This is as a result of skipping 0
        // deltas at the beginning of a batch in 'parse_mouse_deltas'. We don't
        // need to be exact so it's fine and not worth investing in changing.
        let expected_deltas1 = vec![
            DeltaPosition { dx: 1, dy: -1 },
            DeltaPosition { dx: 7, dy: 19 },
            DeltaPosition { dx: 3, dy: 3 },
            DeltaPosition { dx: -3, dy: 2 },
            DeltaPosition { dx: 5, dy: 6 },
        ];
        let expected_deltas2 = vec![
            DeltaPosition { dx: -10, dy: 0 },
            DeltaPosition { dx: 5, dy: 5 },
            DeltaPosition { dx: 10, dy: 9 },
            DeltaPosition { dx: 5, dy: 0 },
            DeltaPosition { dx: 2, dy: 3 },
            DeltaPosition { dx: 5, dy: 13 },
        ];
        check_paths(
            vec![
                (expected_deltas1, 64000 - 11000),
                (expected_deltas2, 254000 - 193500),
            ],
            mouse_paths,
        );
    }
}
