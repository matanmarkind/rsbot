/// Used to develop new actions.
use bot::actions::*;
use screen::{
    action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler, FuzzyPixel,
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

enum Location {
    Draynor,
    VarrockWest,
}

enum Food {
    Shrimp,
    Anchovies,
}

struct Version {
    pub location: Location,
    pub food: Food,
    pub food_bank_slot_index: i32,
    pub wood_bank_slot_index: i32,
}

const VERSION: Version = Version {
    location: Location::VarrockWest,
    food: Food::Anchovies,
    food_bank_slot_index: 2,
    wood_bank_slot_index: 5,
};

fn travel_to_bank() -> TravelTo {
    TravelTo::new(
        /*primary_pixel=*/ fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/
        match VERSION.location {
            Location::Draynor => (0.0, 360.0),
            Location::VarrockWest => (250.0, 180.0),
        },
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ true,
    )
}

fn bank_pixels() -> Vec<FuzzyPixel> {
    match VERSION.location {
        Location::Draynor => vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        Location::VarrockWest => vec![fuzzy_pixels::varrock_bank_window1()],
    }
}

fn deposit_in_bank() -> DepositInBank {
    DepositInBank::new(
        /*expected_pixels=*/
        bank_pixels(),
        /*items=*/
        vec![
            inventory_slot_pixels::oak_logs(),
            inventory_slot_pixels::raw_shrimp_bank(),
            inventory_slot_pixels::cooked_shrimp_bank(),
            inventory_slot_pixels::burned_shrimp_bank(),
            inventory_slot_pixels::raw_anchovies_bank(),
            inventory_slot_pixels::cooked_anchovies_bank(),
        ],
    )
}

fn withdraw_from_bank() -> WithdrawFromBank {
    WithdrawFromBank::new(
        /*bank_pixels=*/
        bank_pixels(),
        /*bank_slot_and_quantity=*/
        vec![
            (VERSION.wood_bank_slot_index, BankQuantity::Exact(2)),
            (VERSION.food_bank_slot_index, BankQuantity::All),
        ],
    )
}

fn travel_to_cooking_spot() -> ExplicitActions {
    ExplicitActions {
        actions: vec![
            Box::new(PressCompass {}),
            Box::new(TravelStraight {
                direction_degrees: match VERSION.location {
                    Location::Draynor => 85.0,
                    Location::VarrockWest => 100.0,
                },
                travel_time: Duration::from_secs(match VERSION.location {
                    Location::Draynor => 9,
                    Location::VarrockWest => 6,
                }),
            }),
        ],
    }
}

fn light_fire() -> ConsumeSingleInventoryItem {
    ConsumeSingleInventoryItem {
        item_to_consume: inventory_slot_pixels::oak_logs(),
        timeout: Duration::from_secs(10),
        actions: vec![
            Box::new(InventorySlotAction::new(inventory_slot_pixels::oak_logs())),
            Box::new(InventorySlotAction::new(inventory_slot_pixels::tinderbox())),
        ],
    }
}

fn cook_fish() -> ConsumeInventory {
    ConsumeInventory {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(10),
        activity_timeout: Duration::from_secs(2 * 60),
        item_to_consume: match VERSION.food {
            Food::Shrimp => inventory_slot_pixels::raw_shrimp(),
            Food::Anchovies => inventory_slot_pixels::raw_anchovies(),
        },
        actions: vec![
            Box::new(InventorySlotAction::new(match VERSION.food {
                Food::Shrimp => inventory_slot_pixels::raw_shrimp(),
                Food::Anchovies => inventory_slot_pixels::raw_anchovies(),
            })),
            Box::new(OpenScreenAction::new(
                /*expected_pixels=*/
                vec![fuzzy_pixels::fire_dark(), fuzzy_pixels::fire_light()],
                /*action_text=*/
                Some(match VERSION.food {
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
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank();
    let deposit_in_bank_actions = deposit_in_bank();
    let withdraw_from_bank_actions = withdraw_from_bank();
    let travel_to_cooking_spot_actions = travel_to_cooking_spot();
    let light_fire_actions = light_fire();
    let cook_fish_actions = cook_fish();

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(1 * 60 * 60) {
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
            std::thread::sleep(Duration::from_secs(5));

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
