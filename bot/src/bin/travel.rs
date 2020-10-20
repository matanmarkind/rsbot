use bot::controller;
use screen::fuzzy_pixels;
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);

    player.open_worldmap();
    std::thread::sleep(Duration::from_secs(1));

    let expected_pixels = vec![fuzzy_pixels::map_icon_bank_yellow()];
    let check_pixels = vec![fuzzy_pixels::map_icon_light_gray()];
    player.travel_to(&expected_pixels, &check_pixels);

    Ok(())
}
