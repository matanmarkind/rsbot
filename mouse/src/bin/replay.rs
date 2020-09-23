use device_query::{DeviceQuery, DeviceState};
use inputbot::MouseCursor;
use mouse::types::*;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::io;
use std::ops::Bound::Included;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.
}

// Struct used to moe the mouse around on the screen.
struct MouseMover {
    // Map of mouse paths to be followed.
    // {PathSummary : MousePath}
    mouse_paths: MousePaths,

    // Used to get the mouse's coordinates. If this doesn't work we can move to
    device_state: DeviceState,
}

impl MouseMover {
    fn new(config: &Config) -> MouseMover {
        MouseMover {
            mouse_paths: bincode::deserialize(&std::fs::read(&config.in_fpath).unwrap()[..])
                .unwrap(),
            device_state: DeviceState::new(),
        }
    }

    // Move the mouse to a point within the box defined by the positions given.
    // Assumes nothing else moves the mouse while running.
    //
    // 'dst' - destination location the mouse should reach.
    //
    // 'tolerance' - size of the box the mouse needs to be within. distance, in
    // pixels, the mouse can be from the exact location in 'dst'.
    fn move_to(&self, dst: &Position, tolerance: u32) {
        let tolerance = tolerance as i32; // Convert for easier use.

        let (x, y) = self.device_state.get_mouse().coords;
        let position = Position { x, y };
        let delta = dst - &position;
        dbg!(&position, &dst, &delta);

        // Create PathSummarys to lookup in mouse_paths. Only the 'distance' field is used for lookup.
        let min_distance = PathSummary {
            distance: std::cmp::max(0, delta.distance() - tolerance),
            avg_time_us: 0,
            angle_rads: 0.0,
        };
        let max_distance = PathSummary {
            distance: delta.distance() + tolerance,
            avg_time_us: 0,
            angle_rads: 0.0,
        };

        // Get an iterator to the relevant paths.
        let mut relevant_paths = self
            .mouse_paths
            .range((Included(&min_distance), Included(&max_distance)));
        match relevant_paths.next() {
            Some((summary, path)) => replay_path(summary, path, &delta),
            None => println!("expand search conditions"),
        }
        dbg!(self.device_state.get_mouse().coords);
    }
}

fn replay_path(summary: &PathSummary, path: &MousePath, net_delta: &DeltaPosition) {
    let rotation_needed = net_delta.angle_rads() - summary.angle_rads;
    dbg!(&rotation_needed, &summary, &path);

    let mut rng = thread_rng();
    let min_wait_us = (0.9 * summary.avg_time_us as f32).round() as i32;
    let max_wait_us = (1.1 * summary.avg_time_us as f32).round() as i32;
    for delta in path {
        // Move the value an absolute distance across the screen (ie num pixels).
        let DeltaPosition { dx, dy } = delta.rotate(rotation_needed);
        MouseCursor::move_abs(dx, dy);
        sleep(Duration::from_micros(
            rng.gen_range(min_wait_us, max_wait_us) as u64,
        ));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mouse_mover = MouseMover::new(&config);
    loop {
        println!("Enter location (x,y,tolerance): ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        println!("{}", buffer);

        // TODO: Find an easier way to deserialize...
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(buffer.as_bytes());
        for result in reader.deserialize::<(Position, u32)>() {
            match result {
                Ok((dst, tolerance)) => {
                    dbg!(&dst, tolerance);
                    mouse_mover.move_to(&dst, tolerance);
                }
                _ => println!("invalid input"),
            }
        }
    }
}
