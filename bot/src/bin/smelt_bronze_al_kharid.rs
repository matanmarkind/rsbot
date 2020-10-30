/*
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionEnableWalk,
    DescribeActionForActionText, DescribeActionForOpenScreen, MousePress, TravelToParams,
};
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Frame};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn travel_to_bank_params() -> TravelToParams {
    TravelToParams {
        destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            // fuzzy_pixels::map_floor_beige(),
            // fuzzy_pixels::black(),
        ],
        starting_direction: None,
        try_to_run: true,
        arc_of_interest: (0.0, 360.0),
    }
}

fn travel_to_furnace_params() -> TravelToParams {
    TravelToParams {
        destination_pixels: vec![
            fuzzy_pixels::map_icon_furnace_yellow(),
            fuzzy_pixels::map_icon_furnace_orange1(),
            fuzzy_pixels::map_icon_furnace_orange2(),
            fuzzy_pixels::map_icon_furnace_gray(),
        ],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_furnace_yellow(),
            fuzzy_pixels::map_icon_furnace_orange1(),
            fuzzy_pixels::map_icon_furnace_orange2(),
            fuzzy_pixels::map_icon_furnace_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_floor_beige(),
            fuzzy_pixels::black(),
        ],
        starting_direction: None,
        try_to_run: true,
        arc_of_interest: (0.0, 360.0),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Add door opening since furnace door can be closed.
    println!("Make sure the set bank Quantity X as 14!!!!");

    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    player.reset();

    let frame = player.capturer.frame().unwrap();

    // Worldmap closed - 98, 101, 77
    // Worldmap open - 80, 91, 80
    // let pixel = frame.get_pixel(&player.framehandler.locations.worldmap_icon());
    // dbg!(pixel);

    // Go to bank
    player.travel_to(&travel_to_bank_params());

    // Deposit bronze bars
    // Withdraw 14 tin & 14 copper
    // Go to furnace

    // player.travel_to(&travel_to_furnace_params());

    // Smelt bronze - use 1 button. Level up will stop us so then continue.

    Ok(())
}
*/

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
