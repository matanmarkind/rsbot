use bot::{
    controller, AwaitAction, ConsumeInventoryOptions, DescribeAction, DescribeActionForActionText,
    DescribeActionForInventory, DescribeActionForMinimap, DescribeActionForOpenScreen, MousePress,
};
use screen::{action_letters, colors};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn fish_small_net_activity() -> ConsumeInventoryOptions {
    ConsumeInventoryOptions {
        multi_slot_action: true,
        timeout: Duration::from_secs(20),
        reset_period: Some(Duration::from_secs(300)),
        inventory_consumption_pixels: vec![colors::INVENTORY_SLOT_EMPTY],
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![colors::SMALL_NET_FISHING_SPOT],
                mouse_press: MousePress::None,
                await_action: AwaitAction::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitAction::Time(Duration::from_nanos(1)),
                action_text: vec![
                    (action_letters::start(), colors::ACTION_WHITE),
                    (action_letters::upper_s(), colors::ACTION_WHITE),
                    (action_letters::lower_m(), colors::ACTION_WHITE),
                    (action_letters::lower_a(), colors::ACTION_WHITE),
                    (action_letters::lower_l(), colors::ACTION_WHITE),
                    (action_letters::lower_l(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::upper_n(), colors::ACTION_WHITE),
                    (action_letters::lower_e(), colors::ACTION_WHITE),
                    (action_letters::lower_t(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::upper_f(), colors::ACTION_YELLOW),
                    (action_letters::lower_i(), colors::ACTION_YELLOW),
                    (action_letters::lower_s(), colors::ACTION_YELLOW),
                    (action_letters::lower_h(), colors::ACTION_YELLOW),
                    (action_letters::lower_i(), colors::ACTION_YELLOW),
                    (action_letters::lower_n(), colors::ACTION_YELLOW),
                    (action_letters::lower_g(), colors::ACTION_YELLOW),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::lower_s(), colors::ACTION_YELLOW),
                    (action_letters::lower_p(), colors::ACTION_YELLOW),
                    (action_letters::lower_o(), colors::ACTION_YELLOW),
                    (action_letters::lower_t(), colors::ACTION_YELLOW),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::forward_slash(), colors::ACTION_WHITE),
                ],
            }),
        ],
    }
}

fn deposit_in_bank() -> ConsumeInventoryOptions {
    ConsumeInventoryOptions {
        multi_slot_action: false,
        timeout: Duration::from_secs(3),
        reset_period: None,
        inventory_consumption_pixels: vec![
            colors::INVENTORY_RAW_SHRIMP_BANK,
            colors::INVENTORY_RAW_ANCHOVIES_BANK,
        ],
        actions: vec![Box::new(DescribeActionForInventory {
            expected_pixels: vec![
                colors::INVENTORY_RAW_SHRIMP_BANK,
                colors::INVENTORY_RAW_ANCHOVIES_BANK,
            ],
            mouse_press: MousePress::Left,
            await_action: AwaitAction::Time(Duration::from_nanos(1)),
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
            expected_pixels: vec![colors::MAP_ICON_BANK_YELLOW],
            check_pixels: vec![colors::MAP_ICON_LIGHT_GRAY],
            mouse_press: MousePress::Left,
            await_action: AwaitAction::Time(Duration::from_secs(20)),
        })];
        player.do_actions(&actions[..]);
        while !player
            .framehandler
            .is_bank_open(&player.capturer.frame().unwrap())
        {
            let actions: Vec<Box<dyn DescribeAction>> = vec![
                Box::new(DescribeActionForOpenScreen {
                    expected_pixels: vec![
                        colors::BANK_BROWN1,
                        colors::BANK_BROWN2,
                        colors::BANK_BROWN3,
                    ],
                    mouse_press: MousePress::None,
                    await_action: AwaitAction::Time(Duration::from_secs(1)),
                }),
                Box::new(DescribeActionForActionText {
                    mouse_press: MousePress::Left,
                    await_action: AwaitAction::IsBankOpen(util::REDRAW_TIME),
                    action_text: vec![
                        (action_letters::start(), colors::ACTION_WHITE),
                        (action_letters::upper_b(), colors::ACTION_WHITE),
                        (action_letters::lower_a(), colors::ACTION_WHITE),
                        (action_letters::lower_n(), colors::ACTION_WHITE),
                        (action_letters::lower_k(), colors::ACTION_WHITE),
                        (action_letters::space(), colors::ACTION_WHITE),
                        (action_letters::upper_b(), colors::ACTION_BLUE),
                        (action_letters::lower_a(), colors::ACTION_BLUE),
                        (action_letters::lower_n(), colors::ACTION_BLUE),
                        (action_letters::lower_k(), colors::ACTION_BLUE),
                        (action_letters::space(), colors::ACTION_WHITE),
                        (action_letters::lower_b(), colors::ACTION_BLUE),
                        (action_letters::lower_o(), colors::ACTION_BLUE),
                        (action_letters::lower_o(), colors::ACTION_BLUE),
                        (action_letters::lower_t(), colors::ACTION_BLUE),
                        (action_letters::lower_h(), colors::ACTION_BLUE),
                        (action_letters::space(), colors::ACTION_WHITE),
                        (action_letters::forward_slash(), colors::ACTION_WHITE),
                    ],
                }),
            ];
            player.do_actions(&actions[..]);
        }
        println!("We're at the bank (I hope).");

        println!("Done depositing.");
        player.consume_inventory(&deposit_in_bank());

        player.reset();
        let actions: Vec<Box<dyn DescribeAction>> = vec![Box::new(DescribeActionForMinimap {
            expected_pixels: vec![colors::MAP_ICON_FISH_DARK_BLUE],
            check_pixels: vec![colors::MAP_ICON_FISH_LIGHT_BLUE],
            mouse_press: MousePress::Left,
            await_action: AwaitAction::Time(Duration::from_secs(20)),
        })];
        player.do_actions(&actions[..]);
    }

    Ok(())
}
