use bot::{
    controller, AwaitFrame, ConsumeInventoryOptions, DescribeAction, DescribeActionForActionText,
    DescribeActionForInventory, DescribeActionForMinimap, DescribeActionForOpenScreen, MousePress,
};
use screen::{
    action_letters, fuzzy_pixels,
    fuzzy_pixels::{action_text_blue, action_text_white, action_text_yellow},
    inventory_slot_pixels,
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn fish_small_net_activity() -> ConsumeInventoryOptions {
    ConsumeInventoryOptions {
        multi_slot_action: true,
        timeout: Duration::from_secs(20),
        reset_period: Some(Duration::from_secs(300)),
        inventory_consumption_pixels: vec![inventory_slot_pixels::empty()],
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::small_net_fishing_spot()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
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

fn deposit_in_bank() -> ConsumeInventoryOptions {
    let inventory_pixels = vec![
        inventory_slot_pixels::raw_shrimp_bank(),
        inventory_slot_pixels::raw_anchovies_bank(),
    ];
    ConsumeInventoryOptions {
        multi_slot_action: false,
        timeout: Duration::from_secs(3),
        reset_period: None,
        inventory_consumption_pixels: inventory_pixels.clone(),
        actions: vec![Box::new(DescribeActionForInventory {
            expected_pixels: inventory_pixels.clone(),
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::Time(Duration::from_nanos(1)),
        })],
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

    // let mut frame = player.capturer.frame().unwrap();
    // for pixels in &[
    //     colors::INVENTORY_RAW_SHRIMP_BANK,
    //     colors::INVENTORY_RAW_ANCHOVIES_BANK,
    // ] {
    //     let first_open_inventory_slot = player
    //         .framehandler
    //         .first_matching_inventory_slot(&frame, pixels);
    //     if !first_open_inventory_slot.is_none() {
    //         dbg!(first_open_inventory_slot);
    //     }
    // }

    // dbg!(player.framehandler.check_inventory_slot(
    //     &player.capturer.frame().unwrap(),
    //     4,
    //     &colors::INVENTORY_RAW_ANCHOVIES_BANK
    // ));
    // let vec: Vec<Box<dyn DescribeAction>> = vec![Box::new(DescribeActionForInventory {
    //     expected_pixels: vec![
    //         colors::INVENTORY_RAW_SHRIMP_BANK,
    //         colors::INVENTORY_RAW_ANCHOVIES_BANK,
    //     ],
    //     mouse_press: MousePress::Left,
    //     await_result_time: Duration::from_secs(1),
    // })];
    // player.do_actions(&vec);

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
        player.reset();
        player.consume_inventory(&fish_small_net_activity());
        println!("Done filling inventory");

        player.reset();
        let actions: Vec<Box<dyn DescribeAction>> = vec![Box::new(DescribeActionForMinimap {
            expected_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
            check_pixels: vec![fuzzy_pixels::map_icon_light_gray()],
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::IsCloseOnMinimapIncomplete(Duration::from_secs(30)),
        })];
        player.do_actions(&actions[..]);

        let actions: Vec<Box<dyn DescribeAction>> = vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![
                    fuzzy_pixels::bank_brown1(),
                    fuzzy_pixels::bank_brown2(),
                    fuzzy_pixels::bank_brown3(),
                ],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::IsBankOpen(Duration::from_secs(2)),
                action_text: vec![
                    (action_letters::start(), action_text_white()),
                    (action_letters::upper_b(), action_text_white()),
                    (action_letters::lower_a(), action_text_white()),
                    (action_letters::lower_n(), action_text_white()),
                    (action_letters::lower_k(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_b(), action_text_blue()),
                    (action_letters::lower_a(), action_text_blue()),
                    (action_letters::lower_n(), action_text_blue()),
                    (action_letters::lower_k(), action_text_blue()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::lower_b(), action_text_blue()),
                    (action_letters::lower_o(), action_text_blue()),
                    (action_letters::lower_o(), action_text_blue()),
                    (action_letters::lower_t(), action_text_blue()),
                    (action_letters::lower_h(), action_text_blue()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::forward_slash(), action_text_white()),
                ],
            }),
        ];
        while !player.do_actions(&actions[..]) {
            // Repeat until we we find the bank successfully since minimap
            // action can quit before we stop walking.
        }

        println!("We're at the bank (I hope).");

        println!("Done depositing.");
        player.consume_inventory(&deposit_in_bank());

        player.reset();
        let actions: Vec<Box<dyn DescribeAction>> = vec![Box::new(DescribeActionForMinimap {
            expected_pixels: vec![fuzzy_pixels::map_icon_fish_dark_blue()],
            check_pixels: vec![fuzzy_pixels::map_icon_fish_light_blue()],
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::IsCloseOnMinimapIncomplete(Duration::from_secs(30)),
        })];
        player.do_actions(&actions[..]);
    }

    Ok(())
}
