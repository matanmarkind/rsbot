use screen::{inventory_slot_pixels, Capturer, FrameHandler};
/// Builds off of mouse_to_pixel. Now we will move the mouse to the desired
/// pixel and left click on it. Instead of a config with a single rectangle
/// bounding the search, we will have multiple rectangles. This is because parts
/// of the screen are covered by the chatbox or the mini map.
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let config = screen::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let framehandler = FrameHandler::new(config.clone());
    let frame = capturer.frame().unwrap();

    // To print out an inventory slot, call check_inventory_slot and uncomment
    // the print statement in the function.
    println!(
        "{}",
        framehandler.check_inventory_slot(&frame, 0, &inventory_slot_pixels::copper_ore())
    );
    return Ok(());

    // framehandler.is_inventory_slot_open(&frame, 2);

    // let empty_slot = framehandler.first_open_inventory_slot(&frame);
    // let shrimp_slot =
    //     framehandler.first_matching_inventory_slot(&frame, &inventory_slot_pixels::raw_shrimp());
    // let anchovies_slot =
    //     framehandler.first_matching_inventory_slot(&frame, &inventory_slot_pixels::raw_anchovies());
    // let tinderbox_slot =
    //     framehandler.first_matching_inventory_slot(&frame, &inventory_slot_pixels::tinderbox());
    // let oak_logs_slot =
    //     framehandler.first_matching_inventory_slot(&frame, &inventory_slot_pixels::oak_logs());
    // println!(
    //     "empty_slot={:?} shrimp_slot={:?} anchovies_slot={:?} tinderbox_slot={:?} oak_logs_slot={:?}",
    //     empty_slot, shrimp_slot, anchovies_slot, tinderbox_slot, oak_logs_slot
    // );

    // Ok(())
}
