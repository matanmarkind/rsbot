/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::{bot_utils, player};
use screen::{
    colors, inventory, letters, locations, locations::TOP_BAR_MIDDLE, ActionLetter, Frame,
    FuzzyPixel, ACTION_BLUE, ACTION_WHITE,
};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use util::*;

fn chop_down_tree_description() -> player::ActionDescription {
    player::ActionDescription {
        colors: vec![colors::TREE_BARK],
        timeout: Duration::from_secs(10),
        multi_item_action: false,
        action_text: vec![
            (letters::upper_c(), ACTION_WHITE),
            (letters::lower_h(), ACTION_WHITE),
            (letters::lower_o(), ACTION_WHITE),
            (letters::lower_p(), ACTION_WHITE),
            (letters::space(), ACTION_WHITE),
            (letters::lower_d(), ACTION_WHITE),
            (letters::lower_o(), ACTION_WHITE),
            (letters::lower_w(), ACTION_WHITE),
            (letters::lower_n(), ACTION_WHITE),
            (letters::space(), ACTION_WHITE),
            (letters::upper_t(), ACTION_BLUE),
            (letters::lower_r(), ACTION_BLUE),
            (letters::lower_e(), ACTION_BLUE),
            (letters::lower_e(), ACTION_BLUE),
            (letters::space(), ACTION_WHITE),
            (letters::forward_slash(), ACTION_WHITE),
        ],
    }
}

fn chop_down_oak_description() -> player::ActionDescription {
    player::ActionDescription {
        colors: vec![colors::OAK_BARK],
        timeout: Duration::from_secs(20),
        multi_item_action: true,
        action_text: vec![
            (letters::upper_c(), ACTION_WHITE),
            (letters::lower_h(), ACTION_WHITE),
            (letters::lower_o(), ACTION_WHITE),
            (letters::lower_p(), ACTION_WHITE),
            (letters::space(), ACTION_WHITE),
            (letters::lower_d(), ACTION_WHITE),
            (letters::lower_o(), ACTION_WHITE),
            (letters::lower_w(), ACTION_WHITE),
            (letters::lower_n(), ACTION_WHITE),
            (letters::space(), ACTION_WHITE),
            (letters::upper_o(), ACTION_BLUE),
            (letters::lower_a(), ACTION_BLUE),
            (letters::lower_k(), ACTION_BLUE),
            (letters::space(), ACTION_WHITE),
            (letters::forward_slash(), ACTION_WHITE),
        ],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = player::Config::from_args();
    dbg!(&config);

    let action_description = chop_down_oak_description();
    let search_locations = vec![
        (
            locations::VERY_NEARBY_SCREEN_TOP_LEFT,
            locations::VERY_NEARBY_SCREEN_DIMENSIONS,
        ),
        (
            locations::NEARBY_SCREEN_TOP_LEFT,
            locations::NEARBY_SCREEN_DIMENSIONS,
        ),
        (
            locations::SCREEN_TOP_LEFT,
            locations::OPEN_SCREEN_DIMENSIONS,
        ),
    ];

    let mut capturer = screen::Capturer::new();
    let mut inputbot = userinput::InputBot::new(&config.mouse_fpath);

    // Bring window into focus.
    while !inputbot.move_near(&TOP_BAR_MIDDLE) {}
    inputbot.left_click();
    bot_utils::close_chatbox(&mut capturer, &mut inputbot);
    bot_utils::open_inventory(&mut inputbot, &capturer.frame().unwrap());

    let time = std::time::Instant::now();
    let mut num_consecutive_misses = 0;
    let looptime = std::time::Instant::now();
    loop {
        if time.elapsed() > Duration::from_secs(60) {
            while !inputbot.move_near(&TOP_BAR_MIDDLE) {}
            inputbot.left_click();
            bot_utils::close_chatbox(&mut capturer, &mut inputbot);
            bot_utils::open_inventory(&mut inputbot, &capturer.frame().unwrap());
        }

        let mut frame = capturer.frame().unwrap();

        let mut first_open_inventory_slot = inventory::first_open_slot(&frame);
        if first_open_inventory_slot.is_none() {
            println!("Inventory is full. Goodbye.");
            return Ok(());
        }

        for (top_left, dimensions) in search_locations.iter() {
            for fuzzy_pixel in action_description.colors.iter() {
                num_consecutive_misses += 1;
                let position =
                    frame.find_pixel_random(&fuzzy_pixel, top_left, &(top_left + dimensions));
                if position.is_none() {
                    println!("{} - no matching pixel", looptime.elapsed().as_secs());
                    continue;
                }

                let position = position.unwrap();
                if !inputbot.move_to(&position) && !inputbot.move_to(&position) {
                    // Try moving the mouse twice since sometimes it is imperfect.
                    println!("{} - couldn't make it :(", looptime.elapsed().as_secs());
                    continue;
                }

                frame = capturer.frame().unwrap();
                if !screen::check_action_letters(&frame, &action_description.action_text[..]) {
                    println!("{} - action didn't match", looptime.elapsed().as_secs());
                    let mut ofpath = config.screenshot_dir.clone();
                    ofpath.push_str(
                        format!(
                            "screenshot_chop_tree_or_oak_{}.png",
                            looptime.elapsed().as_secs()
                        )
                        .as_str(),
                    );
                    screen::mark_letters_and_save(
                        &frame,
                        ofpath.as_str(),
                        &action_description.action_text[..],
                    );
                    continue;
                }

                println!("{} - found it!", looptime.elapsed().as_secs());
                num_consecutive_misses = 0;
                inputbot.left_click();

                let mut waiting_time = std::time::Instant::now();
                while waiting_time.elapsed() < action_description.timeout {
                    sleep(Duration::from_secs(1));
                    frame = capturer.frame().unwrap();
                    let open_slot = inventory::first_open_slot(&frame);
                    if open_slot == first_open_inventory_slot {
                        // Nothing new in the inventory, just keep waiting.
                        continue;
                    }

                    first_open_inventory_slot = open_slot;

                    if !action_description.multi_item_action {
                        // We just received the item we were after, and we can't
                        // continue to receive, so stop waiting for the action to
                        // complete. I would have tried to check here if the
                        // resource is exhausted, but other characters can walk
                        // between the mouse and the object causing us to see them
                        // at that location and mistaking that for resource
                        // exhaustion.
                        break;
                    }

                    // We have received an item so reset the timer to allow us to get more.
                    println!("reset timer for multi_item_action");
                    waiting_time = std::time::Instant::now();
                }

                // Making it here means we succesfully found a resource and
                // clicked it. We will break out and start the loop over to keep
                // preferring searching in the closest location. If we didn't
                // break here we would expand to further boxes for searching.
                break;
            }
        }
        if num_consecutive_misses >= search_locations.len() {
            num_consecutive_misses = 0;
            println!("press left");
            inputbot.pan_left(40.0);
        }
    }
}
