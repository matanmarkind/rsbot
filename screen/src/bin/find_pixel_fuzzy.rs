use screen::{Frame, FuzzyPixel};
use std::error::Error;
use std::io;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        default_value = "0,0",
        about = "top left corner of the image (included). (x,y) represent the top/leftmost row/column of the frame to search in."
    )]
    pub top_left: Position,

    #[structopt(
        long,
        default_value = "1920,1080",
        about = "bottom right of the image (excluded). (x,y) represent one past the bottom/rightmost row/column of the frame to search in."
    )]
    pub dimensions: DeltaPosition,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();

    loop {
        let mut buffer = String::new();

        println!("Enter (blue_min,blue_max,green_min,green_max,red_min,red_max): ");
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let desired_pixel: FuzzyPixel = buffer.trim().parse().unwrap();
        let frame = capturer.frame().unwrap();

        match frame.find_pixel_random(&desired_pixel, &config.top_left, &config.dimensions) {
            Some(pos) => println!("found it! {:?}", pos),
            None => println!("didn't find it :("),
        }
    }
}
