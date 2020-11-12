use bot::actions::*;
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long)]
    pub bronze_bar_bank_slot_index: i32,
}

fn travel_to_bank(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/
        fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ false,
    )
}

fn deposit_in_bank(_config: &Config) -> DepositInBank {
    // TODO: consider dumping the entire inventory. This requires pickaxe is
    // equipped not in inventory.
    DepositInBank::new(
        /*expected_pixels=*/
        vec![fuzzy_pixels::varrock_bank_window1()],
        /*items=*/
        vec![inventory_slot_pixels::bronze_platelegs_bank()],
    )
}

fn withdraw_from_bank(config: &Config) -> WithdrawFromBank {
    WithdrawFromBank::new(
        /*bank_pixels=*/
        vec![fuzzy_pixels::varrock_bank_window1()],
        /*bank_slot_and_quantity=*/
        vec![(config.bronze_bar_bank_slot_index, BankQuantity::All)],
    )
}

fn travel_to_anvil(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_anvil_gray(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::black(),
            fuzzy_pixels::map_floor_brown(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn smith_bronze_platelegs(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(15),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::bronze_bar(),
        actions: vec![
            // Press minimap middle to close the chatbox before clicking 1.
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![
                    fuzzy_pixels::anvil_light_gray(),
                    fuzzy_pixels::anvil_dark_gray(),
                ],
                /*action_text=*/ Some(action_text::smith_anvil()),
                /*mouse_click=*/ MouseClick::Left,
            )),
            // TODO: Add check for is smithing open.
            Box::new(Await {
                condition: AwaitCondition::Time,
                timeout: Duration::from_secs(3),
            }),
            Box::new(PressSmithingPlatelegs {}),
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
    1. We start in the VarrockWest bank with the hammer in our inventory.
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_in_bank_actions = deposit_in_bank(&config);
    let withdraw_from_bank_actions = withdraw_from_bank(&config);
    let travel_to_anvil_actions = travel_to_anvil(&config);
    let smith_bronze_platelegs_actions = smith_bronze_platelegs(&config);

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            deposit_in_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            withdraw_from_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            travel_to_anvil_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = smith_bronze_platelegs_actions.do_action(
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
