use screen::{Frame, Pixel, ACTION_BLUE, ACTION_WHITE};
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

    let letter_and_matchers = [
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
    ];

    // Capture a screenshot, crop it to include just the game window, and flip it to RGB.
    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(screen::check_action_letters(&frame, &letter_and_matchers));

    let mut img = frame.to_owned().flip();

    // Logir here should look like check_action_letters.
    let mut x_offset = screen::TOP_LEFT_ACTION_TEXT.x;
    for (letter, _) in letter_and_matchers.iter() {
        for DeltaPosition { dx, dy } in letter.checkpoints {
            let pos = Position {
                x: x_offset + dx,
                y: screen::TOP_LEFT_ACTION_TEXT.y + dy,
            };
            img.draw_vertical_line(
                &pos,
                1,
                &Pixel {
                    blue: 0,
                    green: 0,
                    red: 255,
                },
            );
        }
        x_offset += letter.width;
    }

    // Save the image.
    let img = img.crop(WINDOW_BOUND.0, WINDOW_BOUND.1);
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
