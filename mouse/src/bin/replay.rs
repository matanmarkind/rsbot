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

const MAX_PATHS_TO_FOLLOW: i32 = 10;
const EXACT_MOVE_DISTANCE: i32 = 10;
const EXPECTED_MOVE_PERIOD_US: u64 = 10000;

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

    // MousePaths is indexed via PathSummary, which only uses distance as the
    // key. Create a min_distance_summary and a max_distance_summary to search
    // for a matching MousePath.
    fn boundary_summaries(distance: i32, tolerance: i32) -> (PathSummary, PathSummary) {
        (
            PathSummary {
                distance: distance - tolerance,
                avg_time_us: 0,
                angle_rads: 0.0,
            },
            PathSummary {
                distance: distance + tolerance,
                avg_time_us: 0,
                angle_rads: 0.0,
            },
        )
    }

    // Move the mouse to a point within the box defined by the positions given.
    // Assumes nothing else moves the mouse while running.
    //
    // 'dst' - destination location the mouse should reach.
    //
    // 'tolerance' - size of the box the mouse needs to be within. Distance, in
    // pixels, that the mouse can be from the exact location in 'dst'. We cannot
    // promise pixel perfect placement.
    //
    // returns the distance the mouse is at the end of the function from 'dst'
    // in pixels.
    fn move_to(&self, dst: &Position) {
        dbg!(&dst);

        for _ in 0..MAX_PATHS_TO_FOLLOW {
            if self.follow_path_to(&dst, EXACT_MOVE_DISTANCE) <= EXACT_MOVE_DISTANCE {
                println!("You made it!");
                break;
            }
        }

        // Once we are close, we can move exactly to the location in 1 move.
        sleep(Duration::from_micros(EXPECTED_MOVE_PERIOD_US));
        self.move_directly_to(&dst);
        sleep(Duration::from_micros(EXPECTED_MOVE_PERIOD_US));
    }

    fn move_directly_to(&self, dst: &Position) {
        let (x, y) = self.device_state.get_mouse().coords;
        let position = Position { x, y };
        let delta = dst - &position;
        MouseCursor::move_abs(delta.dx, delta.dy);
    }

    fn follow_path_to(&self, dst: &Position, tolerance: i32) -> i32 {
        let (x, y) = self.device_state.get_mouse().coords;
        let position = Position { x, y };
        let delta = dst - &position;
        dbg!(&position, &dst, &delta, tolerance);
        if delta.distance() < tolerance {
            // Mouse is already close enough, no need to move.
            return delta.distance();
        }

        // Get an iterator to the relevant paths.
        let (min_distance, max_distance) =
            MouseMover::boundary_summaries(delta.distance(), tolerance);
        let mut relevant_paths = self
            .mouse_paths
            .range((Included(&min_distance), Included(&max_distance)));
        match relevant_paths.next() {
            Some((summary, path)) => replay_path(summary, path, &delta),
            None => {
                // No path brings the mouse close enough. Expand the tolerance
                // and try again.
                self.follow_path_to(&dst, tolerance + tolerance);
            }
        }

        // Calculate the current distance the mouse is from dst.
        let (x, y) = self.device_state.get_mouse().coords;
        let position = Position { x, y };
        let delta = dst - &position;
        delta.distance()
    }
}

fn replay_path(summary: &PathSummary, path: &MousePath, net_delta: &DeltaPosition) {
    let rotation_needed = net_delta.angle_rads() - summary.angle_rads;
    dbg!(&rotation_needed, &summary);

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
        println!("Enter location (x,y): ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        println!("{}", buffer);

        // TODO: Find an easier way to deserialize...
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(buffer.as_bytes());
        for result in reader.deserialize::<Position>() {
            match result {
                Ok(dst) => mouse_mover.move_to(&dst),
                _ => println!("invalid input"),
            }
        }
        dbg!(mouse_mover.device_state.get_mouse().coords);
    }
}
