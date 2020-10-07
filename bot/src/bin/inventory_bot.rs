use screen::{inventory, locations::TOP_BAR_MIDDLE, FuzzyPixel};
use std::error::Error;
use std::thread::sleep;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mouse_fpath: String, // CSV file to read mouse positions from.
}

pub const INVENTORY_OPEN_COLOR: FuzzyPixel = FuzzyPixel {
    blue_min: 25,
    blue_max: 35,
    green_min: 35,
    green_max: 45,
    red_min: 110,
    red_max: 130,
};

// TODO: When leveling up there s a pop up in the chat box. To turn this off ,
// the easiest way is to just left click somewhere in the screen (middle since
// we're always in the middle?)
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let mut inputbot = userinput::InputBot::new(&config.mouse_fpath);

    while !inputbot.move_to(&TOP_BAR_MIDDLE) {}
    inputbot.left_click();

    let mut frame = capturer.frame().unwrap();
    if !inventory::is_inventory_open(&frame) {
        inputbot.click_esc();
        sleep(REDRAW_TIME);
    }

    frame = capturer.frame().unwrap();
    dbg!(inventory::first_open_slot(&frame));

    Ok(())
}
