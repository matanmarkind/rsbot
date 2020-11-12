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
    pub uncooked_pizza_bank_slot_index: i32,
}

fn withdraw_uncooked_pizzas(config: &Config) -> WithdrawFromBank {
    WithdrawFromBank::new(
        /*bank_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*bank_slot_and_quantity=*/
        vec![(config.uncooked_pizza_bank_slot_index, BankQuantity::All)],
    )
}

fn travel_to_cookrange(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_floor_beige(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_cookrange_light_brown(),
            fuzzy_pixels::map_icon_cookrange_medium_brown(),
            fuzzy_pixels::map_icon_cookrange_dark_brown(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::black(),
        ],
        /*arc_of_interest=*/
        (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn cook_pizzas(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: inventory_slot_pixels::uncooked_pizza(),
        actions: vec![
            Box::new(InventorySlotAction::new(
                inventory_slot_pixels::uncooked_pizza(),
            )),
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::cookrange_medium_red()],
                /*action_text=*/
                Some(action_text::use_uncooked_pizza_rightarrow_range()),
                /*mouse_click=*/
                MouseClick::Left,
            )),
            Box::new(ClickChatboxMiddle::new()),
        ],
    }
}

fn travel_to_bank(_config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn deposit_pizzas(_config: &Config) -> DepositInBank {
    // TODO: consider dumping the entire inventory. This requires pickaxe is
    // equipped not in inventory.
    DepositInBank::new(
        /*expected_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*items=*/
        vec![
            inventory_slot_pixels::uncooked_pizza(),
            inventory_slot_pixels::plain_pizza_bank(),
            inventory_slot_pixels::burnt_pizza_bank(),
        ],
    )
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
    1. We start with the inventory full of uncooked pizzas.
    2. We start in the Al Kharid Bank.
    3. Assumes shift click is configured to Use for anchovies (https://github.com/runelite/runelite/wiki/Menu-Entry-Swapper).
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let withdraw_uncooked_pizzas_actions = withdraw_uncooked_pizzas(&config);
    let travel_to_cookrange_actions = travel_to_cookrange(&config);
    let cook_pizzas_actions = cook_pizzas(&config);
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_pizzas_actions = deposit_pizzas(&config);

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(10 * 60) {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            travel_to_cookrange_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        // The door to the cooking range seems to almost always be open and it
        // reopens after a couple minutes if it ever closes. So we assume it is
        // open. We can probably just continue after this and go back to the
        // bank.
        let res = cook_pizzas_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = deposit_pizzas_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = withdraw_uncooked_pizzas_actions.do_action(
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
