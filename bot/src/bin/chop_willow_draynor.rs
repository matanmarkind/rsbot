use bot::actions::*;
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,
}

fn travel_to_bank(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/
        fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn deposit_in_bank(_config: &Config) -> DepositEntireInventoryToBank {
    DepositEntireInventoryToBank::new(/*bank_pixels=*/ vec![
        fuzzy_pixels::bank_brown1(),
        fuzzy_pixels::bank_brown2(),
        fuzzy_pixels::bank_brown3(),
    ])
}

fn travel_to_trees(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_fish_dark_blue(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_icon_fish_light_blue(),
            fuzzy_pixels::map_icon_fish_medium_blue(),
            fuzzy_pixels::map_icon_fish_dark_blue(),
            fuzzy_pixels::black(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn chop_willow(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::empty(),
        actions: vec![
            // Press minimap middle to close the chatbox before clicking 1.
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::willow_bark1(), fuzzy_pixels::willow_bark2()],
                /*action_text=*/ Some(action_text::chop_down_willow()),
                /*mouse_click=*/ MouseClick::Left,
            )),
        ],
    }
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
    1. We start in the VarrockWest bank with the hammer in our inventory..
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_in_bank_actions = deposit_in_bank(&config);
    let travel_to_trees_actions = travel_to_trees(&config);
    let chop_willow_actions = chop_willow(&config);

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            deposit_in_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            travel_to_trees_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = chop_willow_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
