/// Used to develop new actions.
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
    pub copper_ore_bank_slot_index: i32,
    #[structopt(long)]
    pub tin_ore_bank_slot_index: i32,
}

fn travel_to_furnace(_config: &Config) -> TravelTo {
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

fn smelt_bronze(_config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(15),
        activity_timeout: Duration::from_secs(10 * 60),
        item_to_consume: inventory_slot_pixels::copper_ore(),
        actions: vec![
            // Press minimap middle to close the chatbox before clicking 1.
            Box::new(PressMinimapMiddle {}),
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::furnace_grey()],
                /*action_text=*/ Some(action_text::smelt_furnace()),
                /*mouse_click=*/ MouseClick::Left,
            )),
            Box::new(Await {
                condition: AwaitCondition::IsChatboxOpen,
                timeout: Duration::from_secs(3),
            }),
            Box::new(Await {
                condition: AwaitCondition::IsChatboxOpen,
                timeout: Duration::from_millis(500),
            }),
            // TODO: Add WaitChatboxOpen.
            Box::new(ClickKey {
                key: userinput::Key::_1,
            }),
        ],
    }
}

fn travel_to_bank(_config: &Config) -> TravelTo {
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

fn withdraw_from_bank(config: &Config) -> WithdrawFromBank {
    WithdrawFromBank::new(
        /*bank_pixels=*/
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        /*bank_slot_and_quantity=*/
        vec![
            (config.copper_ore_bank_slot_index, BankQuantity::X),
            (config.tin_ore_bank_slot_index, BankQuantity::X),
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
    1. BankQuantity::X is set to 14.
    2. We start in AlKharid bank
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_bars = DepositEntireInventoryToBank::new(/*bank_pixels=*/ vec![
        fuzzy_pixels::bank_brown1(),
        fuzzy_pixels::bank_brown2(),
        fuzzy_pixels::bank_brown3(),
    ]);
    let withdraw_ore = withdraw_from_bank(&config);
    let travel_to_furnace_actions = travel_to_furnace(&config);
    let smelt_iron_actions = smelt_bronze(&config);

    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {
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
        let res = deposit_bars.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = withdraw_ore.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
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
    }

    Ok(())
}
