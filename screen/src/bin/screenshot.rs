// From scrap github repo. Here for my convenience.
use screen::{Frame, FRAME_PERIOD};
use std::io::ErrorKind::WouldBlock;
use std::thread;
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

    let frame;
    loop {
        // Wait until there's a frame.
        match capturer.frame() {
            Ok(f) => {
                frame = f;
                break;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(FRAME_PERIOD);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        }
    }
    let mut subframe = frame.to_owned();
    subframe.crop(Position { x: 100, y: 100 }, Position { x: 500, y: 500 });
    dbg!(&subframe.width, &subframe.height, &subframe.buffer().len());

    println!("Captured! Saving...");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot.png");
    subframe.save(ofpath.as_str());

    // Flip the BGRA image into a RGBA image.
    println!("Flipping...");

    let flipped = subframe.flip();

    // Save the image.
    println!("Saving Flipped...");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_flipped.png");
    flipped.save(ofpath.as_str());
}
