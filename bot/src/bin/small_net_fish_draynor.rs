/// This is a bot for fishing anchovies & shrimp by the Draynor bank.
///
/// Be certain that the bank slots are aligned properly for withdrawal.
use bot::actions::*;
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

fn travel_to_bank() -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
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

fn deposit_in_bank() -> DepositInBank {
    DepositInBank::new(
        /*expected_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*items=*/
        vec![
            inventory_slot_pixels::raw_shrimp_bank(),
            inventory_slot_pixels::raw_anchovies_bank(),
        ],
    )
}

fn travel_to_fishing_spot() -> TravelTo {
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

fn small_net_fish() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(20),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::empty(),
        actions: vec![Box::new(OpenScreenAction::new(
            /*expected_pixels=*/
            vec![fuzzy_pixels::small_net_fishing_spot()],
            /*action_text=*/ Some(action_text::small_net_fishing_spot()),
            /*mouse_click=*/ MouseClick::Left,
        ))],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    let runtime = config.runtime();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank();
    let deposit_in_bank_actions = deposit_in_bank();
    let travel_to_fishing_spot_actions = travel_to_fishing_spot();
    let catch_fish_actions = small_net_fish();

    let time = std::time::Instant::now();
    while time.elapsed() < runtime {
        let reset = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(reset);

        let arrived_at_bank =
            travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(arrived_at_bank);

        let depositted =
            deposit_in_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(depositted);

        let arrived_at_fish = travel_to_fishing_spot_actions.do_action(
            &mut inputbot,
            &mut framehandler,
            &mut capturer,
        );
        dbg!(arrived_at_fish);

        let caught_fish =
            catch_fish_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(caught_fish);
    }

    Ok(())
}
