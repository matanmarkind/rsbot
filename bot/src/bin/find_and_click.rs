/// Builds off of mouse_to_pixel. Now we will move the mouse to the desired
/// pixel and left click on it. Instead of a config with a single rectangle
/// bounding the search, we will have multiple rectangles. This is because parts
/// of the screen are covered by the chatbox or the mini map.
use screen::{Frame, FuzzyPixel};
use std::error::Error;
use std::io;
use structopt::StructOpt;
use util::*;

fn get_pixel_position(
    capturer: &mut screen::Capturer,
    framehandler: &screen::FrameHandler,
) -> Option<Position> {
    let mut buffer = String::new();

    println!("Enter (blue_min,blue_max,green_min,green_max,red_min,red_max): ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let desired_pixel: FuzzyPixel = buffer.trim().parse().unwrap();
    let frame = capturer.frame().unwrap();

    for (top_left, dimensions) in framehandler.locations.open_screen_search_boxes().iter() {
        match frame.find_pixel_random(&desired_pixel, &top_left, &dimensions) {
            Some(pos) => return Some(pos),
            None => (),
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let inputbot = userinput::InputBot::new(config.userinput_config.clone());
    let framehandler = screen::FrameHandler::new(config.screen_config.clone());

    inputbot.move_to(&framehandler.locations.minimap_middle());
    inputbot.left_click();

    loop {
        match get_pixel_position(&mut capturer, &framehandler) {
            Some(pos) => {
                let time = std::time::Instant::now();
                println!("{} - found it! {:?}", time.elapsed().as_millis(), pos);
                inputbot.move_to(&pos);
            }
            None => println!("didn't find it :("),
        }
    }
}
