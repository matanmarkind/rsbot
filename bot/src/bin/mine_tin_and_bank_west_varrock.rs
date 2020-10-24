/// This mining bot was developed for the mines to the west of Port Sarim. The
/// bank is to the northwest.
use bot::{controller, TravelToParams};
use screen::{fuzzy_pixels, inventory_slot_pixels};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn travel_to_bank_params() -> TravelToParams {
    TravelToParams {
        destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_floor_brown(),
        ],
        starting_direction: Some((260.0, Duration::from_secs(25))),
        // starting_direction: None,
        try_to_run: false,
        arc_of_interest: Some((270.0, 45.0)),
    }
}

fn travel_to_mine_params() -> TravelToParams {
    TravelToParams {
        // Use the lighter colors as destination since there are lots of darker
        // grays and browns near the icon.
        destination_pixels: vec![
            fuzzy_pixels::map_icon_pickaxe_light_gray(),
            fuzzy_pixels::map_icon_pickaxe_handle_light_brown(),
            fuzzy_pixels::map_icon_pickaxe_handle_medium_brown(),
        ],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_pickaxe_light_gray(),
            fuzzy_pixels::map_icon_pickaxe_handle_light_brown(),
            fuzzy_pixels::map_icon_pickaxe_handle_medium_brown(),
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        starting_direction: Some((120.0, Duration::from_secs(20))),
        // starting_direction: None,
        try_to_run: false,
        arc_of_interest: Some((0.0, 90.0)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    // player.reset();

    // player.travel_to(&travel_to_bank_params());
    // println!("--- We're at the bank ---");

    // player.deposit_in_bank(
    //     /*bank_colors=*/
    //     &vec![fuzzy_pixels::varrock_bank_window1()],
    //     /*items=*/
    //     &vec![
    //         inventory_slot_pixels::raw_shrimp_bank(),
    //         inventory_slot_pixels::raw_anchovies_bank(),
    //     ],
    // );

    player.travel_to(&travel_to_mine_params());

    Ok(())
}
