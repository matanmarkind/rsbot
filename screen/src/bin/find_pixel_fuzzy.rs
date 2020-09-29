use std::error::Error;
use std::io;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        parse(try_from_str = parse_position),
        default_value = "0,0",
        about = "top left corner of the image (included). (x,y) represent the top/leftmost row/column of the frame to search in."
    )]
    pub top_left: Position,

    #[structopt(
        long,
        parse(try_from_str = parse_position),
        default_value = "1920,1080",
        about = "bottom right of the image (excluded). (x,y) represent one past the bottom/rightmost row/column of the frame to search in."
    )]
    pub past_bottom_right: Position,
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
        let input: Vec<&str> = buffer.trim().split(",").collect();

        let blue_min: u8 = input[0].parse()?;
        let blue_max: u8 = input[1].parse()?;
        let green_min: u8 = input[2].parse()?;
        let green_max: u8 = input[3].parse()?;
        let red_min: u8 = input[4].parse()?;
        let red_max: u8 = input[5].parse()?;

        let desired_bgr_pixel = (
            (blue_min, blue_max),
            (green_min, green_max),
            (red_min, red_max),
        );

        match screen::find_pixel_fuzzy(
            &mut capturer,
            &desired_bgr_pixel,
            &config.top_left,
            &config.past_bottom_right,
        ) {
            Some(pos) => println!("found it! {:?}", pos),
            None => println!("didn't find it :("),
        }
    }
}
