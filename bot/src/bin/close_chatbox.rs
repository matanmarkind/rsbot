use inputbot::MouseButton::LeftButton;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let mouse_mover = mouse::controller::MouseMover::new(&config.in_fpath);

    // Go click on ALL chat button.
    while !mouse_mover.move_to(&ALL_CHAT_BUTTON) {}

    // left click
    LeftButton.press();
    sleep(Duration::from_millis(100));
    LeftButton.release();

    if capturer.check_pixel(&ALL_CHAT_BUTTON, &ALL_CHAT_ON_HIGHLIGHT) {
        println!("It's on!");
        // Go and click once.
        LeftButton.press();
        sleep(Duration::from_millis(100));
        LeftButton.release();
    }

    Ok(())
}
