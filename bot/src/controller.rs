use crate::bot_utils;
use screen::{action_letters::Letter, inventory, locations, Capturer, Frame, FuzzyPixel};
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;
use util::*;

pub struct ActionDescription {
    /// The colors that if found likely correspond with the desired action.
    pub colors: Vec<FuzzyPixel>,

    pub action_text: Vec<(Letter, FuzzyPixel)>,

    /// Can taking this action result in us receiving multiple items over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_item_action: bool,
    /// Amount of time to wait for item to appear in inventory before assuming
    /// we are done (resource exhausted, failed to reach resource, etc.)
    pub timeout: Duration,
}

pub fn get_search_locations() -> Vec<(Position, DeltaPosition)> {
    vec![
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
    ]
}

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

// This is the player class that will tie together the userinput and screen
// crates and wrap them in specific usages.
pub struct Player {
    capturer: Capturer,

    inputbot: InputBot,

    config: Config,
}

impl Player {
    pub fn new(config: Config) -> Player {
        Player {
            capturer: screen::Capturer::new(),
            inputbot: userinput::InputBot::new(config.mouse_fpath.as_str()),
            config,
        }
    }

    /// Closes the chatbox and opens the inventoy. This is the state we want to
    /// perform our loops in.
    pub fn reset(&mut self) {
        while !self.inputbot.move_near(&locations::TOP_BAR_MIDDLE) {}
        self.inputbot.left_click();
        bot_utils::close_chatbox(&mut self.capturer, &mut self.inputbot);
        bot_utils::open_inventory(&mut self.inputbot, &self.capturer.frame().unwrap());
    }

    pub fn fill_inventory(&mut self, action_description: &ActionDescription) {
        let search_locations = get_search_locations();
        self.reset();

        let time = std::time::Instant::now();
        loop {
            if time.elapsed() > Duration::from_secs(60) {
                self.reset();
            }

            let mut frame = self.capturer.frame().unwrap();

            let mut first_open_inventory_slot = inventory::first_open_slot(&frame);
            if first_open_inventory_slot.is_none() {
                println!("Inventory is full. Goodbye.");
                return;
            }

            let mut found_action = false;
            for (top_left, dimensions) in search_locations.iter() {
                for fuzzy_pixel in action_description.colors.iter() {
                    let position = frame.find_pixel_random(&fuzzy_pixel, top_left, &dimensions);
                    if position.is_none() {
                        println!("{} - no matching pixel", time.elapsed().as_secs());
                        continue;
                    }

                    let position = position.unwrap();
                    if !self.inputbot.move_to(&position) && !self.inputbot.move_to(&position) {
                        // Try moving the mouse twice since sometimes it is imperfect.
                        println!("{} - couldn't make it :(", time.elapsed().as_secs());
                        continue;
                    }

                    frame = self.capturer.frame().unwrap();
                    if !screen::action_letters::check_action_letters(
                        &frame,
                        &action_description.action_text[..],
                    ) {
                        println!("{} - action didn't match", time.elapsed().as_secs());
                        let mut ofpath = self.config.screenshot_dir.clone();
                        ofpath.push_str(
                            format!(
                                "screenshot_chop_tree_or_oak_{}.png",
                                time.elapsed().as_secs()
                            )
                            .as_str(),
                        );
                        screen::action_letters::mark_letters_and_save(
                            &frame,
                            ofpath.as_str(),
                            &action_description.action_text[..],
                        );
                        continue;
                    }

                    println!("{} - found it!", time.elapsed().as_secs());
                    found_action = true;
                    self.inputbot.left_click();

                    let mut waiting_time = std::time::Instant::now();
                    while waiting_time.elapsed() < action_description.timeout {
                        sleep(Duration::from_secs(1));
                        frame = self.capturer.frame().unwrap();
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
            if !found_action {
                self.inputbot.pan_left(37.0);
            }
            // Sleep to avoid a busy loop that monopolizes the keyboard and
            // mouse.
            sleep(Duration::from_secs(1));
        }
    }
}
