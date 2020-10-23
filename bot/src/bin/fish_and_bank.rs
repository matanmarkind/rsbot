use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionForActionText,
    DescribeActionForOpenScreen, MousePress, TravelToParams,
};
use screen::{
    action_letters, fuzzy_pixels,
    fuzzy_pixels::{action_text_white, action_text_yellow},
    inventory_slot_pixels,
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn fish_small_net_activity() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        multi_slot_action: true,
        slot_consumption_waittime: Duration::from_secs(20),
        item_to_consume: inventory_slot_pixels::empty(),
        activity_timeout: Duration::from_secs(10 * 60),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::small_net_fishing_spot()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(util::REDRAW_TIME),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_nanos(1)),
                action_text: vec![
                    (action_letters::start(), action_text_white()),
                    (action_letters::upper_s(), action_text_white()),
                    (action_letters::lower_m(), action_text_white()),
                    (action_letters::lower_a(), action_text_white()),
                    (action_letters::lower_l(), action_text_white()),
                    (action_letters::lower_l(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_n(), action_text_white()),
                    (action_letters::lower_e(), action_text_white()),
                    (action_letters::lower_t(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_f(), action_text_yellow()),
                    (action_letters::lower_i(), action_text_yellow()),
                    (action_letters::lower_s(), action_text_yellow()),
                    (action_letters::lower_h(), action_text_yellow()),
                    (action_letters::lower_i(), action_text_yellow()),
                    (action_letters::lower_n(), action_text_yellow()),
                    (action_letters::lower_g(), action_text_yellow()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::lower_s(), action_text_yellow()),
                    (action_letters::lower_p(), action_text_yellow()),
                    (action_letters::lower_o(), action_text_yellow()),
                    (action_letters::lower_t(), action_text_yellow()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::forward_slash(), action_text_white()),
                ],
            }),
        ],
    }
}

/// 1. Catch fish until inventory is full. If full of cooked shrim drop logs
///    until there's only 1 left.
/// 2. Make a fire.
/// 3. Cook all fish in inventory. May take multiple attempts even when
///    selecting all.
/// 4. Drop burned fish
///
fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);

    let time = std::time::Instant::now();
    player.reset();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
        player.travel_to(&TravelToParams {
            destination_pixels: vec![
                fuzzy_pixels::map_icon_fish_light_blue(),
                fuzzy_pixels::map_icon_fish_medium_blue(),
                fuzzy_pixels::map_icon_fish_dark_blue(),
            ],
            confirmation_pixels: vec![
                fuzzy_pixels::map_icon_light_gray(),
                fuzzy_pixels::map_icon_fish_light_blue(),
                fuzzy_pixels::map_icon_fish_medium_blue(),
                fuzzy_pixels::map_icon_fish_dark_blue(),
                fuzzy_pixels::black(),
            ],

            starting_direction: None,
        });
        println!("We are at the fishies");

        player.reset();
        player.consume_inventory(&fish_small_net_activity());
        println!("Done filling inventory");

        player.travel_to(&TravelToParams {
            destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
            confirmation_pixels: vec![
                fuzzy_pixels::map_icon_dark_gray(),
                fuzzy_pixels::map_icon_light_gray(),
            ],

            starting_direction: None,
        });
        println!("We're at the bank (I hope).");

        player.deposit_in_bank(
            /*bank_colors=*/
            &vec![
                fuzzy_pixels::bank_brown1(),
                fuzzy_pixels::bank_brown2(),
                fuzzy_pixels::bank_brown3(),
            ],
            /*items=*/
            &vec![
                inventory_slot_pixels::raw_shrimp_bank(),
                inventory_slot_pixels::raw_anchovies_bank(),
            ],
        );

        // while !player.do_actions(&open_bank_actions()) {
        //     // Repeat until we we find the bank successfully since minimap
        //     // action can quit before we stop walking.
        // }
        // player.consume_inventory(&deposit_in_bank());
        println!("Done depositing.");
    }

    Ok(())
}
