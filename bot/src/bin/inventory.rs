use screen::{colors, Capturer, FrameHandler};
/// Builds off of mouse_to_pixel. Now we will move the mouse to the desired
/// pixel and left click on it. Instead of a config with a single rectangle
/// bounding the search, we will have multiple rectangles. This is because parts
/// of the screen are covered by the chatbox or the mini map.
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let framehandler = FrameHandler::new(config.screen_config.clone());
    let frame = capturer.frame().unwrap();

    // To print out an inventory slot, call check_inventory_slot and uncomment
    // the print statement in the function.
    framehandler.is_inventory_slot_open(&frame, 2);

    let empty_slot = framehandler.first_open_inventory_slot(&frame);
    let shrimp_slot =
        framehandler.first_matching_inventory_slot(&frame, &colors::INVENTORY_RAW_SHRIMP);
    let anchovies_slot =
        framehandler.first_matching_inventory_slot(&frame, &colors::INVENTORY_RAW_ANCHOVIES);
    let tinderbox_slot =
        framehandler.first_matching_inventory_slot(&frame, &colors::INVENTORY_TINDERBOX);
    let oak_logs_slot =
        framehandler.first_matching_inventory_slot(&frame, &colors::INVENTORY_OAK_LOGS);
    println!(
        "empty_slot={:?} shrimp_slot={:?} anchovies_slot={:?} tinderbox_slot={:?} oak_logs_slot={:?}",
        empty_slot, shrimp_slot, anchovies_slot, tinderbox_slot, oak_logs_slot
    );

    Ok(())
}
