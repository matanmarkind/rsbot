use bot::*;
use screen::{inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long, about = "Index of the slot in the bank pizza bases are stored.")]
    pub plain_pizza_bank_slot_index: i32,
    #[structopt(long, about = "Index of the slot in the bank tomatos are stored.")]
    pub anchovy_bank_slot_index: i32,

    #[structopt(long, about = "Which bank we are located in.")]
    pub location: BankLocation,
}

fn withdraw_pizza_and_anchovies(config: &Config) -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/
                bank_pixels(config.location),
                /*bank_slot_and_quantity=*/
                vec![
                    (config.plain_pizza_bank_slot_index, BankQuantity::X),
                    (config.anchovy_bank_slot_index, BankQuantity::X),
                ],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn add_anchovies_to_pizza(_config: &Config) -> ConsumeInventory {
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
            Box::new(Await {
                condition: AwaitCondition::IsChatboxOpen,
                timeout: Duration::from_secs(3),
            }),
            Box::new(ClickKey {
                key: userinput::Key::Space,
            }),
        ],
    }
}

fn deposit_pizzas(config: &Config) -> DepositEntireInventoryToBank {
    // TODO: consider dumping the entire inventory.
    DepositEntireInventoryToBank::new(/*bank_pixels=*/ bank_pixels(config.location))
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
    1. Shift click is configured to Use for anchovies (https://github.com/runelite/runelite/wiki/Menu-Entry-Swapper).
    2. Bank Quantity X is set to 14. 
    3. We are standing in a bank
");

    let reset_actions = ExplicitActions::default_reset();
    let deposit_pizzas_actions = deposit_pizzas(&config);
    let withdraw_pizza_and_anchovies_actions = withdraw_pizza_and_anchovies(&config);
    let add_anchovies_to_pizza_actions = add_anchovies_to_pizza(&config);

    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {
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
