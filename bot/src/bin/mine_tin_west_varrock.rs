/// This mining bot was developed for the mines to the west of Port Sarim. The
/// bank is to the northwest.
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionEnableWalk,
    DescribeActionForActionText, DescribeActionForOpenScreen, MousePress, TravelToParams,
};
use screen::{
    action_letters, fuzzy_pixels,
    fuzzy_pixels::{action_text_blue, action_text_white},
    inventory_slot_pixels,
};
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
        starting_direction: Some((255.0, Duration::from_secs(15))),
        // starting_direction: None,
        try_to_run: false,
        arc_of_interest: Some((270.0, 90.0)),
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
        starting_direction: Some((120.0, Duration::from_secs(15))),
        // starting_direction: None,
        try_to_run: true,
        arc_of_interest: Some((0.0, 90.0)),
    }
}

fn mine_tin_params() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        multi_slot_action: false,
        slot_consumption_waittime: Duration::from_secs(20),
        item_to_consume: inventory_slot_pixels::empty(),
        activity_timeout: Duration::from_secs(10 * 60),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::tin_ore()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(util::REDRAW_TIME),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_nanos(1)),
                action_text: vec![
                    (action_letters::start(), action_text_white()),
                    (action_letters::upper_m(), action_text_white()),
                    (action_letters::lower_i(), action_text_white()),
                    (action_letters::lower_n(), action_text_white()),
                    (action_letters::lower_e(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_r(), action_text_blue()),
                    (action_letters::lower_o(), action_text_blue()),
                    (action_letters::lower_c(), action_text_blue()),
                    (action_letters::lower_k(), action_text_blue()),
                    (action_letters::lower_s(), action_text_blue()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::forward_slash(), action_text_white()),
                ],
            }),
        ],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    player.reset();

    println!("--- Remember to start at the bank with an empty inventory ---");

    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(60 * 60) {
        player.travel_to(&travel_to_mine_params());
        // Get closer to the tin at the north of this mine.
        player.travel_to(&TravelToParams {
            destination_pixels: vec![],
            confirmation_pixels: vec![],
            starting_direction: Some((285.0, Duration::from_secs(5))),
            // starting_direction: None,
            try_to_run: false,
            arc_of_interest: None,
        });
        println!("--- Ready to mine ---");

        // Walk while mining to recover stamina.
        let walk: Vec<Box<dyn DescribeAction>> = vec![DescribeActionEnableWalk::new()];
        player.do_actions(&walk);

        player.reset();
        player.consume_inventory(&mine_tin_params());
        println!("Done filling inventory");

        player.travel_to(&travel_to_bank_params());
        println!("--- We're at the bank ---");

        player.deposit_in_bank(
            /*bank_colors=*/
            &vec![fuzzy_pixels::varrock_bank_window1()],
            /*items=*/
            &vec![inventory_slot_pixels::tin_ore_bank()],
        );
        println!("--- Deposited the ore ---");
    }

    Ok(())
}
