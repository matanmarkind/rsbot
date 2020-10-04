/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use screen::*;
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
    let mouse_mover = mouse::controller::MouseMover::new(&config.mouse_fpath);

    // Bring window into focus.
    while !mouse_mover.move_to(&TOP_BAR_MIDDLE) {}
    mouse::left_click();

    loop {
        let frame = capturer.frame().unwrap();
        let time = std::time::Instant::now();
        match get_pixel_position(&frame, &TREE_PIXEL) {
            Some(pos) => {
                println!(
                    "{} - maybe found it... {:?}",
                    time.elapsed().as_millis(),
                    pos
                );
                if !mouse_mover.move_to(&pos) {
                    println!("{} - couldn't make it :(", time.elapsed().as_millis());
                    continue;
                }
                println!("{} - mouse moved!", time.elapsed().as_millis());

                let frame = capturer.frame().unwrap();
                if !screen::check_action_letters(&frame, CHOP_DOWN_TREE_MATCHERS) {
                    // TODO: learn to rotate camera to recheck.
                    println!("{} - action didn't match", time.elapsed().as_millis());
                    continue;
                }
                println!("{} - found it!", time.elapsed().as_millis());
                mouse::left_click();
                println!("{} - done!", time.elapsed().as_millis());
            }
            None => println!("didn't find it :("),
        }

        // Even once we can monitor the inventory there should be a max timeout
        // and if we reach the max timeout X times in a row there is an issue
        // (perhaps a fence we can't pass). Rotate the screen away.
        sleep(Duration::from_secs(10));
    }
}
