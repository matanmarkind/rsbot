/// Test that combines screen's ability to find pizels from the screen and
/// mouse's ability to move the mouse to a given position.
use screen::{Frame, FuzzyPixel};
use std::error::Error;
use std::io;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.

    #[structopt(
        long,
        default_value = "960,40",
        about = "top left corner of the image (included). (x,y) represent the top/leftmost row/column of the frame to search in."
    )]
    pub top_left: Position,

    #[structopt(
        long,
        default_value = "1920,625",
        about = "bottom right of the image (excluded). (x,y) represent one past the bottom/rightmost row/column of the frame to search in."
    )]
    pub dimensions: DeltaPosition,
}

fn get_pixel_position(config: &Config, capturer: &mut screen::Capturer) -> Option<Position> {
    let mut buffer = String::new();

    println!("Enter (blue_min,blue_max,green_min,green_max,red_min,red_max): ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let desired_pixel: FuzzyPixel = buffer.trim().parse().unwrap();
    let frame = capturer.frame().unwrap();

    frame.find_pixel_random(&desired_pixel, &config.top_left, &config.dimensions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let inputbot = userinput::InputBot::new(&config.in_fpath);

    loop {
        match get_pixel_position(&config, &mut capturer) {
            Some(pos) => {
                let time = std::time::Instant::now();
                println!("{} - found it! {:?}", time.elapsed().as_millis(), pos);
                if inputbot.move_to(&pos) {
                    println!("{} - You made it!", time.elapsed().as_millis());
                } else {
                    println!(
                        "{} - At least you failed valiantly while trying.",
                        time.elapsed().as_millis()
                    );
                }
            }
            None => println!("didn't find it :("),
        }
    }
}
