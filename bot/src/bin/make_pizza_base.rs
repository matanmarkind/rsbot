/// Used to develop new actions.
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

    #[structopt(long)]
    pub jug_of_water_bank_slot_index: i32,
    #[structopt(long)]
    pub pot_of_flour_bank_slot_index: i32,

    #[structopt(long, about = "Which bank we are located in.")]
    pub location: BankLocation,
}

fn deposit_in_bank(config: &Config) -> DepositEntireInventoryToBank {
    DepositEntireInventoryToBank::new(
        /*bank_pixels=*/
        /*bank_pixels=*/ bank_pixels(config.location),
    )
}

fn withdraw_ingredients(config: &Config) -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(WithdrawFromBank::new(
                /*bank_pixels=*/ bank_pixels(config.location),
                /*bank_slot_and_quantity=*/
                vec![
                    (
                        config.pot_of_flour_bank_slot_index,
                        BankQuantity::X,
                        inventory_slot_pixels::pot_of_flour_bank(),
                    ),
                    (
                        config.jug_of_water_bank_slot_index,
                        BankQuantity::X,
                        inventory_slot_pixels::jug_of_water_bank(),
                    ),
                ],
            )),
            Box::new(CloseBank {}),
        ],
    }
}

fn make_pizza_base(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: inventory_slot_pixels::pot_of_flour(),
        actions: vec![
            Box::new(InventorySlotAction::new(
                inventory_slot_pixels::pot_of_flour(),
            )),
            // Likely will not actually pick the first because hover test from
            // the pot of flour will cover it.
            Box::new(InventorySlotAction::new(
                inventory_slot_pixels::jug_of_water(),
            )),
            Box::new(Await {
                condition: AwaitCondition::IsChatboxOpen,
                timeout: Duration::from_secs(3),
            }),
            Box::new(Await {
                condition: AwaitCondition::Time(util::REDRAW_TIME),
                timeout: Duration::from_secs(0),
            }),
            Box::new(ClickKey {
                key: userinput::Key::_3,
            }),
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
    1. We start in a known bank.
    2. BankQuantity::X is set at 9.
"
    );

    let deposit_actions = deposit_in_bank(&config);
    let withdraw_actions = withdraw_ingredients(&config);
    let make_pizza_base_actions = make_pizza_base(&config);

    let res =
        ExplicitActions::default_reset().do_action(&mut inputbot, &mut framehandler, &mut capturer);
    assert!(res);
    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {
        OpenInventory {}.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        let res = deposit_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = withdraw_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            make_pizza_base_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
