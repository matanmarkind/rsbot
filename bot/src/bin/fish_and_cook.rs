use bot::controller;
use screen::{action_letters, colors};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn catch_shrimp_description() -> controller::ActionDescription {
    controller::ActionDescription {
        colors: vec![colors::SMALL_NET_FISHING_SPOT],
        timeout: Duration::from_secs(20),
        multi_item_action: true,
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

    player.fill_inventory(&catch_shrimp_description());
    let frame = player.capturer.frame().unwrap();
    // Find wood in inventory
    let oak_logs_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_OAK_LOGS);
    if oak_logs_slot.is_none() {
        dbg!(oak_logs_slot);
        return Ok(());
    }
    // Find tinderbox in inventory
    let tinderbox_slot = player
        .framehandler
        .first_matching_inventory_slot(&frame, &colors::INVENTORY_TINDERBOX);
    if tinderbox_slot.is_none() {
        dbg!(tinderbox_slot);
        return Ok(());
    }
    // Start fire
    // Find shrimp in inventory
    // Find fire on screen
    // Cook
    // Find anchovies in inventory
    // Find fire on screen
    // Cook

    Ok(())
}
