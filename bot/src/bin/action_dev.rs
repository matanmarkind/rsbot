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
        /*try_to_run=*/ false,
    )
}

// #[derive(Clone)]
// pub enum AwaitFrame {
//     Time(Duration),
//     IsBankOpen(Duration),
//     IsInventoryOpen(Duration),
//     IsWorldMapOpen(Duration),
//     IsWorldMapClosed(Duration),
//     IsChatboxOpen(Duration),

//     IsCloseOnMinimap(Duration, Vec<FuzzyPixel>, Vec<FuzzyPixel>),
//     // Only to be used with DescribeActionForMinimap which converts this to
//     // IsCloseOnMinimap. Otherwise this is the equivalent of Time.
//     IsCloseOnMinimapIncomplete(Duration),
// }
fn smelt_bronze() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: false,
        slot_consumption_waittime: Duration::from_secs(15),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::copper_ore(),
        actions: vec![
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::furnace_grey()],
                /*action_text=*/ Some(action_text::smelt_furnace()),
                /*mouse_click=*/ MouseClick::Left,
            )),
            // TODO: Add WaitChatboxOpen.
            Box::new(ClickKey {
                key: userinput::Key::_1,
            }),
        ],
    }
}

fn travel_to_bank() -> TravelTo {
    // Use map_floor_beige as the primary pizel since clicking directly on the
    // bank yellow can cause us to walk outside the bank (also happened when I
    // manually pressed). I know that coming from the forge starts me on the
    // side of the bank I want to be on, so using the floor biases me to this
    // side.
    TravelTo::new(
        /*primary_pixel=*/
        fuzzy_pixels::map_floor_beige(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_icon_bank_yellow(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ false,
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
    // let travel_to_bank_actions = travel_to_bank();
    // let deposit_bars = DepositEntireInventoryToBank{};
    // Withdraw...
    let travel_to_furnace_actions = travel_to_furnace();
    let smelt_iron_actions = smelt_bronze();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(10 * 60) {
        // let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        // if !res {
        //     dbg!(res);
        //     break;
        // }
        let res =
            travel_to_furnace_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = smelt_iron_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
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
