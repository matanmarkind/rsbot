/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::bot_utils;
use screen::{
    locations::TOP_BAR_MIDDLE, ActionLetter, Frame, FuzzyPixel, ACTION_BLUE, ACTION_WHITE,
};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use util::*;

pub const TREE_PIXEL: FuzzyPixel = FuzzyPixel {
    blue_min: 40,
    blue_max: 44,
    green_min: 81,
    green_max: 85,
    red_min: 114,
    red_max: 118,
};

pub const CHOP_DOWN_TREE_MATCHERS: &[(ActionLetter, FuzzyPixel)] = &[
    (screen::UPPER_C, ACTION_WHITE),
    (screen::LOWER_H, ACTION_WHITE),
    (screen::LOWER_O, ACTION_WHITE),
    (screen::LOWER_P, ACTION_WHITE),
    (screen::SPACE, ACTION_WHITE),
    (screen::LOWER_D, ACTION_WHITE),
    (screen::LOWER_O, ACTION_WHITE),
    (screen::LOWER_W, ACTION_WHITE),
    (screen::LOWER_N, ACTION_WHITE),
    (screen::SPACE, ACTION_WHITE),
    (screen::UPPER_T, ACTION_BLUE),
    (screen::LOWER_R, ACTION_BLUE),
    (screen::LOWER_E, ACTION_BLUE),
    (screen::LOWER_E, ACTION_BLUE),
    (screen::SPACE, ACTION_WHITE),
    (screen::FORWARD_SLASH, ACTION_WHITE),
];

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mouse_fpath: String, // CSV file to read mouse positions from.
}

fn get_pixel_position(frame: &impl Frame, pixel: &FuzzyPixel) -> Option<Position> {
    for BoundingBox {
        0: top_left,
        1: past_bottom_right,
    } in CLEAR_SCREEN_BOUNDS
    {
        match frame.find_pixel_random(pixel, top_left, past_bottom_right) {
            Some(pos) => return Some(pos),
            None => (),
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let mut inputbot = userinput::InputBot::new(&config.mouse_fpath);

    // Bring window into focus.
    while !inputbot.move_near(&TOP_BAR_MIDDLE) {}
    inputbot.left_click();

    bot_utils::close_chatbox(&mut capturer, &mut inputbot);

    let time = std::time::Instant::now();
    let mut num_consecutive_misses = 0;
    loop {
        if time.elapsed() > Duration::from_secs(60) {
            // Once a minute make sure the chatbox is closed.
            bot_utils::close_chatbox(&mut capturer, &mut inputbot);
        }

        let frame = capturer.frame().unwrap();
        let looptime = std::time::Instant::now();
        match get_pixel_position(&frame, &TREE_PIXEL) {
            Some(pos) => {
                println!(
                    "{} - maybe found it... {:?}",
                    looptime.elapsed().as_millis(),
                    pos
                );
                if !inputbot.move_to(&pos) {
                    println!("{} - couldn't make it :(", looptime.elapsed().as_millis());
                    continue;
                }
                println!("{} - mouse moved!", looptime.elapsed().as_millis());

                let frame = capturer.frame().unwrap();
                if !screen::check_action_letters(&frame, CHOP_DOWN_TREE_MATCHERS) {
                    println!("{} - action didn't match", looptime.elapsed().as_millis());
                    num_consecutive_misses += 1;
                    if num_consecutive_misses > 2 {
                        num_consecutive_misses = 0;
                        println!("press left");
                        inputbot.pan_left(90.0);
                    }
                    continue;
                }

                println!("{} - found it!", looptime.elapsed().as_millis());
                num_consecutive_misses = 0;
                inputbot.left_click();
                println!("{} - done!", looptime.elapsed().as_millis());
            }
            None => {
                inputbot.pan_left(90.0);
            }
        }

        // Even once we can monitor the inventory there should be a max timeout
        // and if we reach the max timeout X times in a row there is an issue
        // (perhaps a fence we can't pass). Rotate the screen away.
        sleep(Duration::from_secs(10));
    }
}
