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

// fn travel_to_cookrange() -> TravelTo {
//     TravelTo::new(
//         /*primary_pixel=*/ fuzzy_pixels::map_floor_beige(),
//         /*check_pixels=*/
//         vec![
//             fuzzy_pixels::map_icon_cookrange_light_brown(),
//             fuzzy_pixels::map_icon_cookrange_medium_brown(),
//             fuzzy_pixels::map_icon_cookrange_dark_brown(),
//             fuzzy_pixels::map_icon_light_gray(),
//             fuzzy_pixels::black(),
//         ],
//         /*arc_of_interest=*/
//         (0.0, 360.0),
//         /*timeout=*/ Duration::from_secs(30),
//     )
// }

// fn open_door_actions() -> OpenScreenAction {
//     OpenScreenAction::new(
//         /*expected_pixels=*/
//         vec![
//             fuzzy_pixels::al_kharid_door1(),
//             fuzzy_pixels::al_kharid_door2(),
//             fuzzy_pixels::al_kharid_door3(),
//             fuzzy_pixels::al_kharid_door4(),
//         ],
//         /*action_text=*/ Some(action_text::open_door()),
//         MouseClick::Left,
//     )
// }

fn withdraw_pizza_base_and_tomato() -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/
                vec![
                    fuzzy_pixels::bank_brown1(),
                    fuzzy_pixels::bank_brown2(),
                    fuzzy_pixels::bank_brown3(),
                ],
                /*bank_slot_and_quantity=*/
                vec![(9, -1), (10, -1)],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn make_incomplete_pizza() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: inventory_slot_pixels::pizza_base(),
        actions: vec![
            Box::new(InventorySlotAction {
                item: inventory_slot_pixels::pizza_base(),
                mouse_click: MouseClick::Left,
            }),
            Box::new(InventorySlotAction {
                item: inventory_slot_pixels::tomato(),
                mouse_click: MouseClick::Left,
            }),
            Box::new(ClickChatboxMiddle {}),
        ],
    }
}

fn withdraw_cheese() -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/
                vec![
                    fuzzy_pixels::bank_brown1(),
                    fuzzy_pixels::bank_brown2(),
                    fuzzy_pixels::bank_brown3(),
                ],
                /*bank_slot_and_quantity=*/
                vec![(8, -1)],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn make_uncooked_pizza() -> ConsumeInventory {
    // Uncooked_pizzas look identical to uncooked pizzas.
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(1 * 60),
        item_to_consume: inventory_slot_pixels::cheese(),
        actions: vec![
            Box::new(InventorySlotAction {
                item: inventory_slot_pixels::incomplete_pizza(),
                mouse_click: MouseClick::Left,
            }),
            Box::new(InventorySlotAction {
                item: inventory_slot_pixels::cheese(),
                mouse_click: MouseClick::Left,
            }),
            Box::new(ClickChatboxMiddle {}),
        ],
    }
}

fn deposit_all() -> DepositInBank {
    DepositInBank::new(
        /*expected_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*items=*/
        vec![
            inventory_slot_pixels::uncooked_pizza_bank(),
            inventory_slot_pixels::incomplete_pizza_bank(),
            inventory_slot_pixels::pizza_base_bank(),
            inventory_slot_pixels::tomato_bank(),
            inventory_slot_pixels::cheese_bank(),
        ],
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);

    println!(
        "\
Assumes that:
    1. we start with the inventory empty.
    2. BankQuantityX set to 14.
    3. We are in the bank.
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let deposit_actions = deposit_all();
    let withdraw_pizza_base_and_tomato_actions = withdraw_pizza_base_and_tomato();
    let make_incomplete_pizza_actions = make_incomplete_pizza();
    let withdraw_cheese_actions = withdraw_cheese();
    let make_uncooked_pizza_actions = make_uncooked_pizza();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(10 * 60 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = deposit_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = withdraw_pizza_base_and_tomato_actions.do_action(
            &mut inputbot,
            &mut framehandler,
            &mut capturer,
        );
        if !res {
            dbg!(res);
            break;
        }
        let res = make_incomplete_pizza_actions.do_action(
            &mut inputbot,
            &mut framehandler,
            &mut capturer,
        );
        if !res {
            dbg!(res);
            break;
        }
        let res =
            withdraw_cheese_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            make_uncooked_pizza_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
