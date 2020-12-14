#![allow(dead_code, unused_imports)]

/// Used to develop new actions.
use bot::*;
use screen::{
    action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, Frame, FrameHandler, FuzzyPixel,
};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long)]
    pub jug_of_water_bank_slot_index: i32,
    #[structopt(long)]
    pub pot_of_flour_bank_slot_index: i32,

    #[structopt(long, about = "Which bank we are located in.")]
    pub location: BankLocation,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.bot_config.userinput_config.clone());
    let mut framehandler = FrameHandler::new(config.bot_config.screen_config.clone());
    // Starting with the inventory full of uncooked pizzas is an optimization to
    // avoid putting reset between deposit and withdraw.
    println!(
        "\
Assumes that:
    1. We start in a known bank.
    2. BankQuantity::X is set at 9.
"
    );

    let res =
        ExplicitActions::default_reset().do_action(&mut inputbot, &mut framehandler, &mut capturer);
    assert!(res);
    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {}

    Ok(())
}
