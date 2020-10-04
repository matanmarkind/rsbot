use bot::bot_utils;
use screen::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub in_fpath: String, // CSV file to read mouse positions from.
}

// TODO: When leveling up there s a pop up in the chat box. To turn this off ,
// the easiest way is to just left click somewhere in the screen (middle since
// we're always in the middle?)
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let mouse_mover = mouse::controller::MouseMover::new(&config.in_fpath);

    while !mouse_mover.move_to(&TOP_BAR_MIDDLE) {}
    mouse::left_click();

    bot_utils::close_chatbox(&mut capturer, &mouse_mover);

    Ok(())
}
