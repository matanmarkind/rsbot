use bot::actions::*;
use screen::{
     fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler,
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

pub const PLAIN_PIZZA_BANK_INDEX_SLOT: i32 = 13;
pub const ANCHOVY_BANK_INDEX_SLOT: i32 = 4;

fn withdraw_pizza_and_anchovies() -> ExplicitActions {
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
                vec![
                    (PLAIN_PIZZA_BANK_INDEX_SLOT, BankQuantity::X),
                    (ANCHOVY_BANK_INDEX_SLOT, BankQuantity::X),
                ],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn add_anchovies_to_pizza() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: inventory_slot_pixels::cooked_anchovies(),
        actions: vec![
            Box::new(InventorySlotAction {
                item: inventory_slot_pixels::cooked_anchovies(),
                mouse_click: MouseClick::Left,
                shift_click: true,
            }),
            Box::new(InventorySlotAction::new(
                inventory_slot_pixels::plain_pizza(),
            )),
            Box::new(ClickChatboxMiddle {}),
        ],
    }
}

fn deposit_pizzas() -> DepositInBank {
    // TODO: consider dumping the entire inventory.
    DepositInBank::new(
        /*expected_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*items=*/
        vec![
            inventory_slot_pixels::plain_pizza_bank(),
            inventory_slot_pixels::cooked_anchovies_bank(),
            inventory_slot_pixels::anchovy_pizza_bank(),
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
    1. Shift click is configured to Use for anchovies (https://github.com/runelite/runelite/wiki/Menu-Entry-Swapper).
    2. Bank Quantity X is set to 14. 
    3. Bank slot index for plain pizza is {}
    4. Bank slot index for anchovy is {}
",
        PLAIN_PIZZA_BANK_INDEX_SLOT,
        ANCHOVY_BANK_INDEX_SLOT,
    );

    let reset_actions = ExplicitActions::default_reset();
    let deposit_pizzas_actions = deposit_pizzas();
    let withdraw_pizza_and_anchovies_actions = withdraw_pizza_and_anchovies();
    let add_anchovies_to_pizza_actions = add_anchovies_to_pizza();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(10 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = deposit_pizzas_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = withdraw_pizza_and_anchovies_actions.do_action(
            &mut inputbot,
            &mut framehandler,
            &mut capturer,
        );
        if !res {
            dbg!(res);
            break;
        }
        let res = add_anchovies_to_pizza_actions.do_action(
            &mut inputbot,
            &mut framehandler,
            &mut capturer,
        );
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
