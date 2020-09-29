/// Test that combines screen's ability to find pizels from the screen and
/// mouse's ability to move the mouse to a given position.
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
        parse(try_from_str = parse_position),
        default_value = "960,40",
        about = "top left corner of the image (included). (x,y) represent the top/leftmost row/column of the frame to search in."
    )]
    pub top_left: Position,

    #[structopt(
        long,
        parse(try_from_str = parse_position),
        default_value = "1920,1040",
        about = "bottom right of the image (excluded). (x,y) represent one past the bottom/rightmost row/column of the frame to search in."
    )]
    pub past_bottom_right: Position,
}

fn get_pixel_position(config: &Config, mut capturer: &mut screen::Capturer) -> Option<Position> {
    let mut buffer = String::new();

    println!("Enter (blue_min,blue_max,green_min,green_max,red_min,red_max): ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let input: Vec<&str> = buffer.trim().split(",").collect();

    let blue_min: u8 = input[0].parse().unwrap();
    let blue_max: u8 = input[1].parse().unwrap();
    let green_min: u8 = input[2].parse().unwrap();
    let green_max: u8 = input[3].parse().unwrap();
    let red_min: u8 = input[4].parse().unwrap();
    let red_max: u8 = input[5].parse().unwrap();

    let desired_bgr_pixel = (
        (blue_min, blue_max),
        (green_min, green_max),
        (red_min, red_max),
    );

    screen::find_pixel_fuzzy(
        &mut capturer,
        &desired_bgr_pixel,
        &config.top_left,
        &config.past_bottom_right,
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let mouse_mover = mouse::controller::MouseMover::new(&config.in_fpath);

    loop {
        match get_pixel_position(&config, &mut capturer) {
            Some(pos) => {
                println!("found it! {:?}", pos);
                if mouse_mover.move_to(&pos) {
                    println!("You made it!");
                } else {
                    println!("At least you failed valiantly while trying.");
                }
            }
            None => println!("didn't find it :("),
        }
    }
}
