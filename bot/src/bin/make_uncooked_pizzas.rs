use bot::actions::*;
use screen::{fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

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
                vec![(9, BankQuantity::X), (10, BankQuantity::X)],
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
            Box::new(InventorySlotAction::new(inventory_slot_pixels::pizza_base())),
            Box::new(InventorySlotAction::new(inventory_slot_pixels::tomato())),
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
                vec![(8, BankQuantity::X)],
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
            Box::new(InventorySlotAction::new(
                inventory_slot_pixels::incomplete_pizza(),
            )),
            Box::new(InventorySlotAction::new(inventory_slot_pixels::cheese())),
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
