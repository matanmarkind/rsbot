/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use inputbot::MouseButton::LeftButton;
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

    let frame = capturer.frame().unwrap();

    loop {
        match get_pixel_position(&frame, &TREE_PIXEL) {
            Some(pos) => {
                let time = std::time::Instant::now();
                println!("{} - found it! {:?}", time.elapsed().as_millis(), pos);
                if mouse_mover.move_to(&pos) {
                    println!("{} - You made it!", time.elapsed().as_millis());
                    LeftButton.press();
                    sleep(Duration::from_millis(50));
                    LeftButton.release();
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
