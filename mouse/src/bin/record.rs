// This file is dedicated to recording mouse movements.
// Tracks the location of the mouse as it moves on the screen and saves it to a CSV.
use device_query::{DeviceQuery, DeviceState};
use mouse::constants::*;
use mouse::types::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::num::ParseIntError;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

fn parse_duration_from_secs(src: &str) -> Result<Duration, ParseIntError> {
    let seconds: u64 = src.parse()?;
    Ok(Duration::from_secs(seconds))
}

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long, about = "File to append CSV recording to")]
    pub out_fpath: String, // Serialized output of mouse paths.

    #[structopt(
        long,
        parse(try_from_str = parse_duration_from_secs),
        default_value = "1",
        about = "Period between writing out recordings to a file (seconds)."
    )]
    pub batch_period_s: Duration,

    #[structopt(
        long,
        parse(try_from_str = parse_duration_from_secs),
        default_value = "1",
        about = "Period between writing out recordings to a file (seconds)."
    )]
    pub active_time_s: Duration,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let device_state = DeviceState::new();
    let start_time = std::time::Instant::now();

    let num_batch_iters =
        (config.batch_period_s.as_micros() / MIN_TIME_BETWEEN_LOCATIONS.as_micros()) as i32;
    let mut locations = vec![ZERO_LOC; num_batch_iters as usize];

    // Create a CSV writer to append locations to the existing file.
    let output_file = OpenOptions::new().append(true).open(&config.out_fpath)?;
    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(output_file);

    while start_time.elapsed() < config.active_time_s {
        for loc in locations.iter_mut() {
            let (x, y) = device_state.get_mouse().coords;
            *loc = Location {
                time_us: start_time.elapsed().as_micros() as i64,
                x,
                y,
            };
            sleep(MIN_TIME_BETWEEN_LOCATIONS);
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
