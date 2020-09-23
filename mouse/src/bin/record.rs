// This file is dedicated to recording mouse movements.
// Tracks the location of the mouse as it moves on the screen and saves it to a CSV.
use device_query::{DeviceQuery, DeviceState};
use mouse::types::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::Duration;

use structopt::StructOpt;

// TODO: convert to using Duration.
#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long, about = "File to append CSV recording to")]
    pub out_fpath: String, // Serialized output of mouse paths.

    // 'sampling_period' offers a tradeoff. The higher rate creates more
    // mouselike movement to avoid teleporting mouse detection. The downside is
    // that the recording takes time so the shorter we make this the less stable
    // the actual sampling period becomes. We face the same problem on the
    // replay end, since the usage has to be able to keep up with the rate being
    // fed in. It also means more data so more memory.
    #[structopt(
        long,
        parse(try_from_str),
        default_value = "9",
        about = "Time to wait between recording mouse position (milliseconds)."
    )]
    pub sampling_period_ms: i64,

    #[structopt(
        long,
        parse(try_from_str),
        default_value = "1",
        about = "Period between writing out recordings to a file (seconds)."
    )]
    pub batch_period_s: i64,

    #[structopt(
        long,
        parse(try_from_str),
        default_value = "1",
        about = "Period between writing out recordings to a file (seconds)."
    )]
    pub active_time_s: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let device_state = DeviceState::new();
    let start_time = std::time::Instant::now();

    let num_batch_iters =
        (config.batch_period_s as f32 * 1000.0 / config.sampling_period_ms as f32).round() as i32;
    let mut locations = vec![ZERO_LOC; num_batch_iters as usize];

    // Create a CSV writer to append locations to the existing file.
    let output_file = OpenOptions::new().append(true).open(&config.out_fpath)?;
    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(output_file);

    while start_time.elapsed().as_secs() < config.active_time_s as u64 {
        for loc in locations.iter_mut() {
            let (x, y) = device_state.get_mouse().coords;
            *loc = Location {
                time_us: start_time.elapsed().as_micros() as i64,
                x,
                y,
            };
            sleep(Duration::from_millis(config.sampling_period_ms as u64));
        }
        // dbg!(&locations);
        let mut time_deltas = Vec::<_>::new();
        let mut last_time = locations[0].time_us;
        for Location {
            time_us,
            x: _,
            y: _,
        } in locations.iter()
        {
            time_deltas.push(time_us - last_time);
            last_time = *time_us;
        }
        let bad_deltas: Vec<_> = time_deltas
            .iter()
            .filter(|&&x| x < 9000 || x > 11000)
            .collect();
        dbg!(locations.len(), &bad_deltas);

        // Write the csv. Begin each batch with (0,0,0)
        csv_writer.serialize(Location {
            time_us: 0,
            x: 0,
            y: 0,
        })?;
        for loc in locations.iter() {
            csv_writer.serialize(loc)?;
        }
    }
    Ok(())
}
