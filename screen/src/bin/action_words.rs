/// Take a screenshot of the game and draw lines to separate the characters in
/// the text that describes an action. This is a test to see if they are regular.
use std::fs::File;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a slash (e.g. /path/to/dir/ on linux)"
    )]
    pub out_dir: String,
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();

    // Capture a screenshot, crop it to include just the game window, and flip it to RGB.
    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(frame.check_first_action_letter(&screen::C));

    let mut img = frame.flip();
    for DeltaPosition { dx, dy } in screen::C.checkpoints {
        let pos = Position {
            x: screen::TOP_LEFT_ACTION_TEXT.x + dx,
            y: screen::TOP_LEFT_ACTION_TEXT.y + dy,
        };
        img = img.draw_vertical_line(&pos, 1, (0, 255, 0));
    }

    // Save the image.
    let img = img.subframe(WINDOW_BOUND.0, WINDOW_BOUND.1);
    println!("Saving...");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot.png");
    repng::encode(
        File::create(&ofpath).unwrap(),
        img.width as u32,
        img.height as u32,
        img.buffer(),
    )
    .unwrap();
}
