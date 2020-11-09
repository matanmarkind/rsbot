use bot::actions::*;
use screen::{fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler, FuzzyPixel};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use strum_macros::EnumString;
use userinput::InputBot;

#[derive(Debug, Copy, Clone, EnumString)]
pub enum Location {
    AlKharid,
    Falador,
    VarrockWest,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long, about = "Index of the slot in the bank pizza bases are stored.")]
    pub pizza_base_bank_slot_index: i32,
    #[structopt(long, about = "Index of the slot in the bank tomatos are stored.")]
    pub tomato_bank_slot_index: i32,
    #[structopt(long, about = "Index of the slot in the bank chesse are stored.")]
    pub cheese_bank_slot_index: i32,

    #[structopt(long, about = "Which bank we are located in.")]
    pub location: Location,
}

fn bank_pixels(loc: Location) -> Vec<FuzzyPixel> {
    match loc {
        Location::AlKharid => vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        Location::Falador => vec![
            fuzzy_pixels::falador_bank_brown1(),
            fuzzy_pixels::falador_bank_brown2(),
        ],
        Location::VarrockWest => vec![fuzzy_pixels::varrock_bank_window1()],
    }
}

fn withdraw_pizza_base_and_tomato(config: &Config) -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/ bank_pixels(config.location),
                /*bank_slot_and_quantity=*/
                vec![
                    (config.pizza_base_bank_slot_index, BankQuantity::X),
                    (config.tomato_bank_slot_index, BankQuantity::X),
                ],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn make_incomplete_pizza(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: inventory_slot_pixels::pizza_base(),
        actions: vec![
            Box::new(InventorySlotAction::new(inventory_slot_pixels::pizza_base())),
            Box::new(InventorySlotAction::new(inventory_slot_pixels::tomato())),
            Box::new(ClickChatboxMiddle::new()),
        ],
    }
}

fn withdraw_cheese(config: &Config) -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/ bank_pixels(config.location),
                /*bank_slot_and_quantity=*/
                vec![(config.cheese_bank_slot_index, BankQuantity::X)],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn make_uncooked_pizza(_config: &Config) -> ConsumeInventory {
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
            Box::new(ClickChatboxMiddle::new()),
        ],
    }
}

fn deposit_all(config: &Config) -> DepositEntireInventoryToBank {
    DepositEntireInventoryToBank::new(/*bank_pixels=*/ bank_pixels(config.location))
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.bot_config.userinput_config.clone());
    let mut framehandler = FrameHandler::new(config.bot_config.screen_config.clone());

    println!(
        "\
Assumes that:
    1. we start with the inventory empty.
    2. BankQuantityX set to 14.
    3. We are in the bank.
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let deposit_actions = deposit_all(&config);
    let withdraw_pizza_base_and_tomato_actions = withdraw_pizza_base_and_tomato(&config);
    let make_incomplete_pizza_actions = make_incomplete_pizza(&config);
    let withdraw_cheese_actions = withdraw_cheese(&config);
    let make_uncooked_pizza_actions = make_uncooked_pizza(&config);

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
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
