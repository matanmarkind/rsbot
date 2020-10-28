/// This mining bot was developed for the mines to the west of Port Sarim. The
/// bank is to the northwest.
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeAction, DescribeActionEnableWalk,
    DescribeActionForActionText, DescribeActionForOpenScreen, MousePress, TravelToParams,
};
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn travel_to_bank_params() -> TravelToParams {
    TravelToParams {
        destination_pixels: vec![fuzzy_pixels::map_icon_bank_yellow()],
        confirmation_pixels: vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
            fuzzy_pixels::map_floor_gray(),
        ],
        starting_direction: Some((285.0, Duration::from_secs(10))),
        // starting_direction: None,
        try_to_run: false,
        arc_of_interest: (0.0, 360.0),
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
        starting_direction: Some((0.0, Duration::from_secs(15))),
        // starting_direction: None,
        try_to_run: true,
        arc_of_interest: (10.0, 160.0),
    }
}

fn mine_copper_params() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        multi_slot_action: false,
        slot_consumption_waittime: Duration::from_secs(10),
        item_to_consume: inventory_slot_pixels::empty(),
        activity_timeout: Duration::from_secs(6 * 60),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::copper_ore()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(util::REDRAW_TIME),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_nanos(1)),
                action_text: action_text::mine_rocks(),
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
        // Go north west a bit to go to usually less crowded rocks.
        player.travel_to(&TravelToParams {
            destination_pixels: vec![],
            confirmation_pixels: vec![],
            starting_direction: Some((200.0, Duration::from_secs(3))),
            try_to_run: false,
            arc_of_interest: (0.0, 360.0),
        });
        println!("--- Ready to mine ---");

        // Walk while mining to recover stamina.
        let walk: Vec<Box<dyn DescribeAction>> = vec![DescribeActionEnableWalk::new()];
        player.do_actions(&walk);

        player.reset();
        player.consume_inventory(&mine_copper_params());
        println!("Done filling inventory");

        player.travel_to(&travel_to_bank_params());
        println!("--- We're at the bank ---");

        player.deposit_in_bank(
            /*bank_colors=*/
            &vec![fuzzy_pixels::varrock_bank_window1()],
            /*items=*/
            &vec![
                inventory_slot_pixels::copper_ore_bank(),
                inventory_slot_pixels::tin_ore_bank(),
                inventory_slot_pixels::silver_ore_bank(),
                inventory_slot_pixels::iron_ore_bank(),
                inventory_slot_pixels::clay_bank(),
                inventory_slot_pixels::uncut_sapphire_bank(),
                inventory_slot_pixels::uncut_ruby_bank(),
            ],
        );
        println!("--- Deposited the ore ---");
    }

    Ok(())
}
