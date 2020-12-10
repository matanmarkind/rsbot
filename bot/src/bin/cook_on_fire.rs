/// Used to develop new actions.
use bot::actions::*;
use screen::{
    action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler, FuzzyPixel,
    InventorySlotPixels,
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use strum_macros::EnumString;
use userinput::InputBot;

#[derive(Debug, Copy, Clone, EnumString)]
pub enum Location {
    Draynor,
    VarrockWest,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum Food {
    Shrimp,
    Anchovies,
}

#[derive(Debug, Copy, Clone, EnumString)]
pub enum Logs {
    Tree,
    Oak,
    Willow,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long)]
    pub location: Location,
    #[structopt(long)]
    pub logs: Logs,
    #[structopt(long)]
    pub food: Food,

    #[structopt(long)]
    pub food_bank_slot_index: i32,
    #[structopt(long)]
    pub logs_bank_slot_index: i32,
}

fn get_logs_inventory_pixel(config: &Config) -> InventorySlotPixels {
    match config.logs {
        Logs::Tree => inventory_slot_pixels::tree_logs(),
        Logs::Oak => inventory_slot_pixels::oak_logs(),
        Logs::Willow => inventory_slot_pixels::willow_logs(),
    }
}

fn travel_to_bank(config: &Config) -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/
        match config.location {
            Location::Draynor => (0.0, 360.0),
            Location::VarrockWest => (250.0, 180.0),
        },
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn bank_pixels(config: &Config) -> Vec<FuzzyPixel> {
    match config.location {
        Location::Draynor => vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        Location::VarrockWest => vec![fuzzy_pixels::varrock_bank_window1()],
    }
}

fn deposit_in_bank(config: &Config) -> DepositInBank {
    DepositInBank::new(
        /*expected_pixels=*/
        bank_pixels(config),
        /*items=*/
        vec![
            inventory_slot_pixels::tree_logs(),
            inventory_slot_pixels::oak_logs(),
            inventory_slot_pixels::willow_logs(),
            inventory_slot_pixels::raw_shrimp_bank(),
            inventory_slot_pixels::cooked_shrimp_bank(),
            inventory_slot_pixels::burned_shrimp_bank(),
            inventory_slot_pixels::raw_anchovies_bank(),
            inventory_slot_pixels::cooked_anchovies_bank(),
        ],
    )
}

fn withdraw_from_bank(config: &Config) -> WithdrawFromBank {
    WithdrawFromBank::new(
        /*bank_pixels=*/
        bank_pixels(config),
        /*bank_slot_and_quantity=*/
        vec![
            (config.logs_bank_slot_index, BankQuantity::Exact(2)),
            (config.food_bank_slot_index, BankQuantity::All),
        ],
    )
}

fn travel_to_cooking_spot(config: &Config) -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(PressCompass {}),
            Box::new(TravelStraight {
                direction_degrees: match config.location {
                    Location::Draynor => 85.0,
                    Location::VarrockWest => 100.0,
                },
                travel_time: Duration::from_secs(match config.location {
                    Location::Draynor => 10,
                    Location::VarrockWest => 6,
                }),
            }),
        ],
    }
}

fn light_fire(config: &Config) -> ConsumeSingleInventoryItem {
    ConsumeSingleInventoryItem {
        item_to_consume: get_logs_inventory_pixel(config),
        timeout: Duration::from_secs(10),
        actions: vec![
            Box::new(InventorySlotAction::new(get_logs_inventory_pixel(config))),
            Box::new(InventorySlotAction::new(inventory_slot_pixels::tinderbox())),
        ],
    }
}

fn cook_fish(config: &Config) -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        // Takes about 2m, but this is only checked if we stop consuming slots.
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: match config.food {
            Food::Shrimp => inventory_slot_pixels::raw_shrimp(),
            Food::Anchovies => inventory_slot_pixels::raw_anchovies(),
        },
        actions: vec![
            Box::new(InventorySlotAction::new(match config.food {
                Food::Shrimp => inventory_slot_pixels::raw_shrimp(),
                Food::Anchovies => inventory_slot_pixels::raw_anchovies(),
            })),
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::fire_dark(), fuzzy_pixels::fire_light()],
                /*action_text=*/
                Some(match config.food {
                    Food::Shrimp => action_text::use_raw_shrimp_rightarrow_fire(),
                    Food::Anchovies => action_text::use_raw_anchovies_rightarrow_fire(),
                }),
                /*mouse_click=*/
                MouseClick::Left,
            )),
            Box::new(ClickChatboxMiddle::new()),
        ],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.bot_config.userinput_config.clone());
    let mut framehandler = FrameHandler::new(config.bot_config.screen_config.clone());

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_in_bank_actions = deposit_in_bank(&config);
    let withdraw_from_bank_actions = withdraw_from_bank(&config);
    let travel_to_cooking_spot_actions = travel_to_cooking_spot(&config);
    let light_fire_actions = light_fire(&config);
    let cook_fish_actions = cook_fish(&config);

    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {
        let reset = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(reset);

        let arrived_at_bank =
            travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(arrived_at_bank);

        let depositted =
            deposit_in_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(depositted);

        let withdrew =
            withdraw_from_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        dbg!(withdrew);

        let time = std::time::Instant::now();
        while time.elapsed() < Duration::from_secs(5 * 60) {
            let travelled = travel_to_cooking_spot_actions.do_action(
                &mut inputbot,
                &mut framehandler,
                &mut capturer,
            );
            dbg!(travelled);

            let lit_fire =
                light_fire_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
            dbg!(lit_fire);
            if !lit_fire {
                // We failed to attempt lighting a fire, so we are out of wood.
                break;
            }

            // wait for fire to start.
            std::thread::sleep(Duration::from_secs(7));

            let cooked =
                cook_fish_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
            dbg!(cooked);
            if cooked {
                // We cooked everything, so we are done.
                break;
            }
        }
    }

    Ok(())
}
