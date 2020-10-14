/// Test that combines screen's ability to find pizels from the screen and
/// mouse's ability to move the mouse to a given position.
use screen::{Frame, FuzzyPixel};
use std::error::Error;
use std::io;
use structopt::StructOpt;
use util::*;

fn get_pixel_position(config: &bot::Config, capturer: &mut screen::Capturer) -> Option<Position> {
    let mut buffer = String::new();

    println!("Enter (blue_min,blue_max,green_min,green_max,red_min,red_max): ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let desired_pixel: FuzzyPixel = buffer.trim().parse().unwrap();
    let frame = capturer.frame().unwrap();

    frame.find_pixel_random(
        &desired_pixel,
        &config.screen_config.screen_top_left,
        &(&config.screen_config.screen_bottom_right - &config.screen_config.screen_top_left),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let inputbot = userinput::InputBot::new(config.userinput_config.clone());

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
