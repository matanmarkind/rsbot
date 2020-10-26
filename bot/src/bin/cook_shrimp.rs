/// This is a bot for cooking raw shrimp on oak fire by the Draynor bank.
///
/// Make sure you already have the tinderbox in your inventory, the bank is
/// scrolled up, and the shrimp and wood are in the right slots.
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionForActionText,
    DescribeActionForInventory, DescribeActionForOpenScreen, DescribeActionPressChatboxMiddle,
    MousePress, TravelToParams,
};
use screen::fuzzy_pixels::{action_text_blue, action_text_orange, action_text_white};
use screen::{action_letters, fuzzy_pixels, inventory_slot_pixels, FuzzyPixel};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

/// Either Draynor or Varrock west bank.
const DRAYNOR: bool = false;

fn cook_shrimp_consume_options() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(5),
        item_to_consume: inventory_slot_pixels::raw_shrimp(),
        activity_timeout: Duration::from_secs(2 * 60),
        actions: vec![
            Box::new(DescribeActionForInventory {
                expected_pixels: vec![inventory_slot_pixels::raw_shrimp()],
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
                action_text: vec![
                    (action_letters::start(), action_text_white()),
                    (action_letters::upper_u(), action_text_white()),
                    (action_letters::lower_s(), action_text_white()),
                    (action_letters::lower_e(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_r(), action_text_orange()),
                    (action_letters::lower_a(), action_text_orange()),
                    (action_letters::lower_w(), action_text_orange()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::lower_s(), action_text_orange()),
                    (action_letters::lower_h(), action_text_orange()),
                    (action_letters::lower_r(), action_text_orange()),
                    (action_letters::lower_i(), action_text_orange()),
                    (action_letters::lower_m(), action_text_orange()),
                    (action_letters::lower_p(), action_text_orange()),
                    (action_letters::lower_s(), action_text_orange()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::hyphen(), action_text_white()),
                    (action_letters::greater_than(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_f(), action_text_blue()),
                    (action_letters::lower_i(), action_text_blue()),
                    (action_letters::lower_r(), action_text_blue()),
                    (action_letters::lower_e(), action_text_blue()),
                ],
            }),
            DescribeActionPressChatboxMiddle::new(),
        ],
    }
}

fn get_bank_booth_pixels() -> Vec<FuzzyPixel> {
    if DRAYNOR {
        vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ]
    } else {
        vec![fuzzy_pixels::varrock_bank_window1()]
    }
}

fn explicit_walk_time() -> Option<(f32, Duration)> {
    if DRAYNOR {
        Some((85.0, Duration::from_secs(11)))
    } else {
        Some((95.0, Duration::from_secs(6)))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    let time = std::time::Instant::now();
    while time.elapsed() < Duration::from_secs(1 * 60 * 60) {
        player.reset();
        player.travel_to(&TravelToParams {
            destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
            confirmation_pixels: vec![
                fuzzy_pixels::map_icon_dark_gray(),
                fuzzy_pixels::map_icon_light_gray(),
                fuzzy_pixels::black(),
            ],

            try_to_run: false,
        arc_of_interest: (0.0, 360.0),
            starting_direction: None,
        });
        println!("--- We're at the bank ---");

        player.deposit_in_bank(
            &get_bank_booth_pixels(),
            /*items=*/
            &vec![
                inventory_slot_pixels::oak_logs(),
                inventory_slot_pixels::raw_shrimp_bank(),
                inventory_slot_pixels::cooked_shrimp_bank(),
                inventory_slot_pixels::burned_shrimp_bank(),
            ],
        );
        println!("--- Made the deposit ---");

        player.withdraw_from_bank(
            /*bank_colors=*/
            &vec![
                fuzzy_pixels::bank_brown1(),
                fuzzy_pixels::bank_brown2(),
                fuzzy_pixels::bank_brown3(),
            ], /*bank_slot_and_quantity:=*/
            // Withdraw 1 log and the rest shrimp.
            &vec![(5, 2), (1, 0)],
        );
        println!("--- We got the wood and shrimp ---");

        let fire_start_time = std::time::Instant::now();
        while fire_start_time.elapsed() < Duration::from_secs(3 * 60) {
            player.travel_to(&TravelToParams {
                try_to_run: false,
        arc_of_interest: (0.0, 360.0),
                destination_pixels: vec![],
                confirmation_pixels: vec![],
                starting_direction: explicit_walk_time(),
            });
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
                if player.consume_inventory(&cook_shrimp_consume_options()) {
                    // Unfortunately every other time that we click on the shrimp we
                    // unselect it...
                    println!("--- Ick Shrimp! ---");
                    break;
                }
            }
            println!("--- We were unable to cook all the food :( ---");
        }
    }

    Ok(())
}
