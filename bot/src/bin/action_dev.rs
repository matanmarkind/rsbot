/// Used to develop new actions.
use bot::controller;
use screen::fuzzy_pixels;
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    player.withdraw_from_bank(
        /*bank_colors=*/
        &vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ], /*bank_slot_and_quantity:=*/
        &vec![(1, 3), (5, 1), (2, 0)],
    );

    Ok(())
}
