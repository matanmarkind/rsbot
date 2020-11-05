/// Used to develop new actions.
use bot::actions::*;
use screen::{
    action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler, FuzzyPixel,
};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

fn travel_to_furnace() -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_furnace_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_furnace_orange1(),
            fuzzy_pixels::map_icon_furnace_orange2(),
            fuzzy_pixels::map_icon_furnace_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_floor_beige(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
    )
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

fn travel_to_bank() -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_floor_beige(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
    )
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

    // let reset_actions = ExplicitActions::default_reset();
    let travel_to_furnace_actions = travel_to_furnace();
    // let mine_copper_actions = mine_copper();
    let travel_to_bank_actions = travel_to_bank();
    // let deposit_copper_actions = deposit_copper();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(1) {
        // let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        // if !res {
        //     dbg!(res);
        //     break;
        // }
        let res = travel_to_furnace_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        // let res = mine_copper_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        // if !res {
        //     dbg!(res);
        //     break;
        // }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        // let res = deposit_copper_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        // if !res {
        //     dbg!(res);
        //     break;
        // }
    }

    Ok(())
}
