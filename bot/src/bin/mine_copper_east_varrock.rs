use bot::actions::*;
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

fn travel_to_mine() -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            // No press compass since this is right after reset.
            Box::new(TravelStraight {
                direction_degrees: 0.0,
                travel_time: Duration::from_secs(20),
            }),
            Box::new(TravelStraight {
                direction_degrees: 60.0,
                travel_time: Duration::from_secs(10),
            }),
            Box::new(TravelTo::new(
                /*primary_pixel=*/ fuzzy_pixels::map_icon_pickaxe_dark_gray(),
                /*check_pixels=*/
                vec![
                    fuzzy_pixels::map_icon_pickaxe_light_gray(),
                    fuzzy_pixels::map_icon_pickaxe_handle_light_brown(),
                    fuzzy_pixels::map_icon_pickaxe_handle_medium_brown(),
                    fuzzy_pixels::map_icon_dark_gray(),
                    fuzzy_pixels::map_icon_light_gray(),
                ],
                /*arc_of_interest=*/ (0.0, 360.0),
                /*timeout=*/ Duration::from_secs(60),
                /*try_to_run=*/ true,
            )),
            // Usually these rocks are less crowded.
            // Box::new(TravelStraight {
            //     direction_degrees: 245.0,
            //     travel_time: Duration::from_secs(5),
            // }),
            // Box::new(PressMinimapMiddle {}),
        ],
    }
}

fn mine_copper() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: false,
        slot_consumption_waittime: Duration::from_secs(15),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::empty(),
        actions: vec![Box::new(OpenScreenAction::new(
            /*expected_pixels=*/
            vec![fuzzy_pixels::copper_ore()],
            /*action_text=*/ Some(action_text::mine_rocks()),
            /*mouse_click=*/ MouseClick::Left,
        ))],
    }
}

fn travel_to_bank() -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(PressCompass {}),
            Box::new(TravelStraight {
                direction_degrees: 290.0,
                travel_time: Duration::from_secs(10),
            }),
            Box::new(TravelTo::new(
                /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
                /*check_pixels=*/
                vec![
                    fuzzy_pixels::map_icon_dark_gray(),
                    fuzzy_pixels::map_icon_light_gray(),
                    fuzzy_pixels::map_floor_brown(),
                ],
                /*arc_of_interest=*/ (0.0, 360.0),
                /*timeout=*/ Duration::from_secs(60),
                /*try_to_run=*/ true,
            )),
        ],
    }
}

fn deposit_copper() -> DepositInBank {
    // TODO: consider dumping the entire inventory. This requires pickaxe is
    // equipped not in inventory.
    DepositInBank::new(
        /*expected_pixels=*/
        vec![fuzzy_pixels::varrock_bank_window1()],
        /*items=*/
        vec![
            inventory_slot_pixels::copper_ore_bank(),
            inventory_slot_pixels::silver_ore_bank(),
            inventory_slot_pixels::iron_ore_bank(),
            inventory_slot_pixels::clay_bank(),
            inventory_slot_pixels::uncut_sapphire_bank(),
            inventory_slot_pixels::uncut_ruby_bank(),
        ],
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);
    // Starting with the inventory full of uncooked pizzas is an optimization to
    // avoid putting reset between deposit and withdraw.
    println!(
        "\
Assumes that:
    1. We can't go through the Workman's gate.
    2. We start at the bank with pickaxe equipped. 
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_mine_actions = travel_to_mine();
    let mine_copper_actions = mine_copper();
    let travel_to_bank_actions = travel_to_bank();
    let deposit_copper_actions = deposit_copper();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(2 * 60 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_mine_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = mine_copper_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = deposit_copper_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
