use screen::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mouse_fpath: String, // CSV file to read mouse positions from.
}

pub const ICON_RED: FuzzyPixel = FuzzyPixel {
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
    inputbot.pan_left(90.0);

    let frame = capturer.frame().unwrap();
    println!(
        "open? {}",
        ICON_RED.matches(&frame.get_pixel(&INVENTORY_ICON))
    );
    inputbot.click_esc();

    let frame = capturer.frame().unwrap();
    println!(
        "open? {}",
        ICON_RED.matches(&frame.get_pixel(&INVENTORY_ICON))
    );

    Ok(())
}
