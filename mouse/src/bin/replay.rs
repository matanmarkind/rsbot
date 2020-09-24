use mouse::types::*;
use std::error::Error;
use std::io;
use structopt::StructOpt;

extern crate mouse;
use mouse::controller;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mouse_mover = controller::MouseMover::new(&config.in_fpath);
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
                Ok(dst) => {
                    if mouse_mover.move_to(&dst) {
                        println!("You made it!");
                    } else {
                        println!("At least you failed valiantly while trying.");
                    }
                }
                _ => println!("invalid input"),
            }
            println!("current_position={:?}", mouse_mover.current_position());
        }
    }
}
