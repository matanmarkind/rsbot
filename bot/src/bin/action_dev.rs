/// Used to develop new actions.
use bot::actions::*;
use screen::{fuzzy_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);

    // let action = TravelStraight {
    //     direction_degrees: 75.0,
    //     travel_time: Duration::from_secs(11),
    // };

    let action = TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
    );

    // let action = MaybeToggleWorldmap::open_worldmap();
    let res = action.do_action(&mut inputbot, &mut framehandler, &mut capturer);
    dbg!(res);
    Ok(())
}
