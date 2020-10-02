use std::error::Error;
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

    while !mouse_mover.move_to(&TOP_BAR) {}
    mouse::left_click();

    if !capturer.check_pixels(&[
        CHAT_BOX_TOP_LEFT,
        CHAT_BOX_BOTTOM_LEFT,
        CHAT_BOX_TOP_RIGHT,
        CHAT_BOX_BOTTOM_RIGHT,
    ]) {
        return Ok(());
    }
    // Go click on the All tab
    while !mouse_mover.move_to(&ALL_CHAT_BUTTON) {}
    mouse::left_click();

    // If a different tab was seleected (not All) then the All tab will now be open. Close it.
    std::thread::sleep(std::time::Duration::from_millis(200));
    let mut capturer = screen::Capturer::new();
    if capturer.check_pixels(&[
        CHAT_BOX_TOP_LEFT,
        CHAT_BOX_BOTTOM_LEFT,
        CHAT_BOX_TOP_RIGHT,
        CHAT_BOX_BOTTOM_RIGHT,
    ]) {
        // This is never happening since the top left keeps appearing as 114,137,147.
        mouse::left_click();
    }

    Ok(())
}
