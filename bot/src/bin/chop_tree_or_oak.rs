/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::bot_utils;
use screen::{
    colors, inventory, letters, locations, locations::TOP_BAR_MIDDLE, ActionLetter, Frame,
    FuzzyPixel, ACTION_BLUE, ACTION_WHITE,
};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use util::*;

struct ActionDescription {
    /// The colors that if found likely correspond with the desired action.
    pub colors: Vec<FuzzyPixel>,

    pub action_text: Vec<(ActionLetter, FuzzyPixel)>,

    /// Can taking this action result in us receiving multiple items over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_item_action: bool,
    /// Amount of time to wait for item to appear in inventory before assuming
    /// we are done (resource exhausted, failed to reach resource, etc.)
    pub timeout: Duration,
}

fn chop_down_tree_description() -> ActionDescription {
    ActionDescription {
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

// Max timeout for an item to appear in the inventory before assuming chopping
// down the tree has failed.
pub const CHOP_DOWN_OAK_TIMEOUT: Duration = Duration::from_secs(10);

// pub const CHOP_DOWN_OAK_ACTION_TEXT: &[(ActionLetter, FuzzyPixel)] = &[
//     (screen::UPPER_C, ACTION_WHITE),
//     (screen::LOWER_H, ACTION_WHITE),
//     (screen::LOWER_O, ACTION_WHITE),
//     (screen::LOWER_P, ACTION_WHITE),
//     (screen::SPACE, ACTION_WHITE),
//     (screen::LOWER_D, ACTION_WHITE),
//     (screen::LOWER_O, ACTION_WHITE),
//     (screen::LOWER_W, ACTION_WHITE),
//     (screen::LOWER_N, ACTION_WHITE),
//     (screen::SPACE, ACTION_WHITE),
//     (screen::UPPER_O, ACTION_BLUE),
//     (screen::LOWER_A, ACTION_BLUE),
//     (screen::LOWER_K, ACTION_BLUE),
//     (screen::SPACE, ACTION_WHITE),
//     (screen::FORWARD_SLASH, ACTION_WHITE),
// ];

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mouse_fpath: String, // CSV file to read mouse positions from.
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a slash (e.g. /path/to/dir/ on linux)"
    )]
    pub screenshot_dir: String,
}

fn get_pixel_position(frame: &impl Frame, pixel: &FuzzyPixel) -> Option<Position> {
    for (top_left, dimensions) in &[
        (
            locations::NEARBY_SCREEN_TOP_LEFT,
            locations::NEARBY_SCREEN_DIMENSIONS,
        ),
        (
            locations::SCREEN_TOP_LEFT,
            locations::OPEN_SCREEN_DIMENSIONS,
        ),
    ] {
        match frame.find_pixel_random(pixel, top_left, &(top_left + dimensions)) {
            Some(pos) => return Some(pos),
            None => (),
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let chop_down_tree_desc = chop_down_tree_description();

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
            // Once a minute make sure the chatbox is closed.
            bot_utils::close_chatbox(&mut capturer, &mut inputbot);
        }

        for fuzzy_pixel in chop_down_tree_desc.colors.iter() {
            num_consecutive_misses += 1;
            let frame = capturer.frame().unwrap();

            let mut first_open_inventory_slot = inventory::first_open_slot(&frame);
            if first_open_inventory_slot.is_none() {
                println!("Inventory is full. Goodbye.");
                return Ok(());
            }

            let pos = get_pixel_position(&frame, &fuzzy_pixel);
            if pos.is_none() {
                println!("{} - no matching pixel", looptime.elapsed().as_secs());
                continue;
            }
            let pos = pos.unwrap();
            if !inputbot.move_to(&pos) && !inputbot.move_to(&pos) {
                // Try moving the mouse twice since sometimes it is imperfect.
                println!("{} - couldn't make it :(", looptime.elapsed().as_secs());
                continue;
            }
            println!("{} - mouse moved!", looptime.elapsed().as_secs());

            let frame = capturer.frame().unwrap();
            if !screen::check_action_letters(&frame, &chop_down_tree_desc.action_text[..]) {
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
                    &chop_down_tree_desc.action_text[..],
                );
                continue;
            }

            println!("{} - found it!", looptime.elapsed().as_secs());
            num_consecutive_misses = 0;
            inputbot.left_click();
            let mut waiting_time = std::time::Instant::now();
            while waiting_time.elapsed() < chop_down_tree_desc.timeout {
                sleep(Duration::from_secs(1));
                let frame = capturer.frame().unwrap();
                let open_slot = inventory::first_open_slot(&frame);
                if open_slot == first_open_inventory_slot {
                    continue;
                }

                if !chop_down_tree_desc.multi_item_action {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete.
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                println!("reset timer for multi_item_action");
                first_open_inventory_slot = open_slot;
                waiting_time = std::time::Instant::now();
            }
        }
        if num_consecutive_misses > 2 {
            num_consecutive_misses = 0;
            println!("press left");
            inputbot.pan_left(90.0);
        }
    }
}
