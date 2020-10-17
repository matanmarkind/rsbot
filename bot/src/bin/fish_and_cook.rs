use bot::{
    controller, Activity, DescribeActionForActionText,
    DescribeActionForOpenScreen, FillInventory, MousePress,
};
use screen::{action_letters, colors};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn fish_small_net_activity() -> FillInventory {
    FillInventory {
        multi_item_action: true,
        timeout: Duration::from_secs(20),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![colors::SMALL_NET_FISHING_SPOT],
                mouse_press: MousePress::None,
                await_result_time: Duration::from_nanos(1),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_result_time: Duration::from_nanos(1),
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
    let fill_inventory = fish_small_net_activity();
    fill_inventory.do_activity(&mut player);
    println!("Done filling inventory");

    /*
    let mut frame = player.capturer.frame().unwrap();
    let oak_logs_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_OAK_LOGS);
    if oak_logs_slot.is_none() {
        dbg!(oak_logs_slot);
        return Ok(());
    }

    let tinderbox_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_TINDERBOX);
    if tinderbox_slot.is_none() {
        dbg!(tinderbox_slot);
        return Ok(());
    }

    let shrimp_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_RAW_SHRIMP);
    if shrimp_slot.is_none() {
        dbg!(shrimp_slot);
        return Ok(());
    }

    // Start fire
    player.press_inventory_slot(oak_logs_slot.unwrap());
    player.press_inventory_slot(tinderbox_slot.unwrap());

    // Find shrimp in inventory and click
    player.press_inventory_slot(shrimp_slot.unwrap());

    // Find fire on screen and click - probably need to right click.
    // Fire starting can fail.
    std::thread::sleep(Duration::from_secs(5));
    frame = player.capturer.frame().unwrap();
    for (top_left, dimensions) in player
        .framehandler
        .locations
        .open_screen_search_boxes()
        .iter()
    {
        for fuzzy_pixel in [colors::FIRE_OPAQUE, colors::FIRE_TRANSLUCENT].iter() {
            let position = frame.find_pixel_random(&fuzzy_pixel, top_left, &dimensions);
            if position.is_none() {
                println!("no matching pixel");
                continue;
            }

            let position = position.unwrap();
            player.inputbot.move_to(&position);
        }
    }
    player.inputbot.left_click();

    // Cook - probably fastest to always pick 1 so we can immediately move onto
    // the next 1. That is likely a give for botting so give all and quickstop
    // when we cook everything but also have timeout in case of level up.
    player
        .inputbot
        .move_to(&player.framehandler.locations.chatbox_middle());
    player.inputbot.left_click();
    while !player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_RAW_SHRIMP)
        .is_none()
    {
        std::thread::sleep(Duration::from_secs(2));
        frame = player.capturer.frame().unwrap();
        // TODO: figure out how to close the chatbox if it is opened (due to
        // level up?). Perhaps we can just ignore it and repress?
    }
    println!("All the shrimp are cooked");

    // Find anchovies in inventory
    let anchovies_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_RAW_ANCHOVIES);
    if anchovies_slot.is_none() {
        dbg!(anchovies_slot);
        return Ok(());
    }
    player.press_inventory_slot(anchovies_slot.unwrap());

    // Find fire on screen

    // Cook

    // Drop burned ones.

    */
    Ok(())
}
