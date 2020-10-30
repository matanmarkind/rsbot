/// Used to develop new actions.
use bot::actions::*;
use screen::{Capturer, FrameHandler};
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

    let walk_north = TravelStraight {
        direction_degrees: 270.0,
        travel_time: Duration::from_secs(11),
    };
    walk_north.do_action(&mut inputbot, &mut framehandler, &mut capturer);

    Ok(())
}
