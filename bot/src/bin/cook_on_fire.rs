/// This is a bot for cooking raw shrimp on oak fire by the Draynor bank.
///
/// Make sure you already have the tinderbox in your inventory, the bank is
/// scrolled up, and the shrimp and wood are in the right slots.
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionForActionText,
    DescribeActionForInventory, DescribeActionForOpenScreen, DescribeActionPressChatboxMiddle,
    MousePress, TravelToParams,
};
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, ActionText, FuzzyPixel};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

/// Either Draynor or Varrock west bank.
enum BankLocation {
    Draynor,
    VarrockWest,
}

enum Food {
    Shrimp,
    Anchovies,
}

const VERSION: (BankLocation, Food) = (BankLocation::VarrockWest, Food::Anchovies);
const FOOD_BANK_SLOT_INDEX: i32 = 2;
const WOOD_BANK_SLOT_INDEX: i32 = 5;

fn get_raw_inventory_pixels() -> screen::InventorySlotPixels {
    match VERSION.1 {
        Food::Shrimp => inventory_slot_pixels::raw_shrimp(),
        Food::Anchovies => inventory_slot_pixels::raw_anchovies(),
    }
}

fn get_action_text() -> ActionText {
    match VERSION.1 {
        Food::Shrimp => action_text::use_raw_shrimp_rightarrow_fire(),
        Food::Anchovies => action_text::use_raw_anchovies_rightarrow_fire(),
    }
}

fn get_bank_booth_pixels() -> Vec<FuzzyPixel> {
    match VERSION.0 {
        BankLocation::Draynor => vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        BankLocation::VarrockWest => vec![fuzzy_pixels::varrock_bank_window1()],
    }
}

fn travel_to_bank_params() -> TravelToParams {
    TravelToParams {
        destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::black(),
        ],

        try_to_run: false,
        arc_of_interest: match VERSION.0 {
            BankLocation::Draynor => (0.0, 360.0),
            BankLocation::VarrockWest => (250.0, 280.0),
        },
        starting_direction: None,
    }
}

fn travel_to_cooking_spot_params() -> TravelToParams {
    TravelToParams {
        try_to_run: false,
        arc_of_interest: (0.0, 360.0),
        destination_pixels: vec![],
        confirmation_pixels: vec![],
        starting_direction: match VERSION.0 {
            BankLocation::Draynor => Some((85.0, Duration::from_secs(11))),
            BankLocation::VarrockWest => Some((95.0, Duration::from_secs(6))),
        },
    }
}

fn cook_food_consumption_params() -> ConsumeInventoryParams {
    let inventory_pixels = get_raw_inventory_pixels();
    let action_text = get_action_text();
    ConsumeInventoryParams {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(5),
        item_to_consume: inventory_pixels.clone(),
        activity_timeout: Duration::from_secs(90),
        actions: vec![
            Box::new(DescribeActionForInventory {
                expected_pixels: vec![inventory_pixels.clone()],
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::fire_dark(), fuzzy_pixels::fire_light()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(util::REDRAW_TIME),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::IsChatboxOpen(Duration::from_secs(3)),
                action_text: action_text,
            }),
            DescribeActionPressChatboxMiddle::new(),
        ],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    let time = std::time::Instant::now();
    while time.elapsed() < Duration::from_secs(1 * 60 * 60) {
        player.reset();
        player.travel_to(&travel_to_bank_params());
        println!("--- We're at the bank ---");

        player.deposit_in_bank(
            &get_bank_booth_pixels(),
            /*items=*/
            &vec![
                inventory_slot_pixels::oak_logs(),
                inventory_slot_pixels::raw_shrimp_bank(),
                inventory_slot_pixels::cooked_shrimp_bank(),
                inventory_slot_pixels::burned_shrimp_bank(),
                inventory_slot_pixels::raw_anchovies_bank(),
                inventory_slot_pixels::cooked_anchovies_bank(),
            ],
        );
        println!("--- Made the deposit ---");

        player.withdraw_from_bank(
            /*bank_colors=*/
            &get_bank_booth_pixels(),
            // Withdraw 1 log and the rest food.
            /*bank_slot_and_quantity:=*/
            &vec![(WOOD_BANK_SLOT_INDEX, 2), (FOOD_BANK_SLOT_INDEX, 0)],
        );
        println!("--- We got the wood and food ---");

        let fire_start_time = std::time::Instant::now();
        while fire_start_time.elapsed() < Duration::from_secs(3 * 60) {
            player.travel_to(&travel_to_cooking_spot_params());
            println!("--- Get a fire going! ---");

            // In a loop light fire then comsume_inventory(cook_shrim).
            let mut start_fire_actions = Vec::<Box<dyn DescribeAction>>::new();
            start_fire_actions.push(Box::new(DescribeActionForInventory {
                expected_pixels: vec![inventory_slot_pixels::oak_logs()],
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(util::REDRAW_TIME),
            }));
            start_fire_actions.push(Box::new(DescribeActionForInventory {
                expected_pixels: vec![inventory_slot_pixels::tinderbox()],
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(5)),
            }));

            if player.do_actions(&start_fire_actions) {
                println!("--- FIRE! ---");
                if player.consume_inventory(&cook_food_consumption_params()) {
                    // Unfortunately every other time that we click on the shrimp we
                    // unselect it...
                    println!("--- Yum! ---");
                    break;
                }
            }
            println!("--- We were unable to cook all the food :( ---");
        }
    }

    Ok(())
}
