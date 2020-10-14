use screen::{action_letters::Letter, Capturer, Frame, FrameHandler, FuzzyPixel};
use std::thread::sleep;
use std::time::Duration;
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

// This is the player class that will tie together the userinput and screen
// crates and wrap them in specific usages.
pub struct Player {
    capturer: Capturer,

    framehandler: FrameHandler,

    inputbot: InputBot,
}

impl Player {
    pub fn new(config: crate::Config) -> Player {
        Player {
            capturer: Capturer::new(),
            inputbot: InputBot::new(config.userinput_config.clone()),
            framehandler: FrameHandler::new(config.screen_config.clone()),
        }
    }

    /// Closes the chatbox and opens the inventoy. This is the state we want to
    /// perform our loops in.
    pub fn reset(&mut self) {
        while !self
            .inputbot
            .move_near(&self.framehandler.locations.minimap_middle())
        {}
        self.inputbot.left_click();
        self.open_inventory();
        self.close_chatbox();
    }

    // Assumes runelight is the active screen.
    pub fn open_inventory(&mut self) {
        let frame = self.capturer.frame().unwrap();
        if self.framehandler.is_inventory_open(&frame) {
            // dbg!("frame already open");
            return;
        }
        self.inputbot.click_esc();

        // std::thread::sleep(util::REDRAW_TIME);
        // frame = self.capturer.frame().unwrap();
        // dbg!(self.framehandler.is_inventory_open(&frame));
    }

    pub fn fill_inventory(&mut self, action_description: &ActionDescription) {
        self.reset();

        let time = std::time::Instant::now();
        loop {
            if time.elapsed() > Duration::from_secs(300) {
                self.reset();
            }

            let mut frame = self.capturer.frame().unwrap();

            let mut first_open_inventory_slot = self.framehandler.first_open_inventory_slot(&frame);
            if first_open_inventory_slot.is_none() {
                println!("Inventory is full. Goodbye.");
                return;
            }

            let mut found_action = false;
            for (top_left, dimensions) in self
                .framehandler
                .locations
                .open_screen_search_boxes()
                .iter()
            {
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
                    if !self
                        .framehandler
                        .check_action_letters(&frame, &action_description.action_text[..])
                    {
                        println!("{} - action didn't match", time.elapsed().as_secs());
                        continue;
                    }

                    println!("{} - found it!", time.elapsed().as_secs());
                    found_action = true;
                    self.inputbot.left_click();

                    let mut waiting_time = std::time::Instant::now();
                    while waiting_time.elapsed() < action_description.timeout {
                        sleep(Duration::from_secs(1));
                        frame = self.capturer.frame().unwrap();
                        let open_slot = self.framehandler.first_open_inventory_slot(&frame);
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
                if found_action {
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

    fn close_chatbox(&mut self) {
        let mut frame = self.capturer.frame().unwrap();
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        while !self
            .inputbot
            .move_near(&self.framehandler.locations.all_chat_button())
        {}
        self.inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        frame = self.capturer.frame().unwrap();

        // If the chatbox is still open it's possible a different chat tab was
        // selected and now the ALL tab is on.
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        while !self
            .inputbot
            .move_near(&self.framehandler.locations.all_chat_button())
        {}
        self.inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        frame = self.capturer.frame().unwrap();

        // If the chatbox is still open this is likely due to an update such as
        // leveling up. This closes by left clicking most things
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        } // Click the center of the minimap since this will only move us a small
          // amount. Safest/easiest way I could think of torandomly left click.
        while !self
            .inputbot
            .move_near(&self.framehandler.locations.minimap_middle())
        {}
        self.inputbot.left_click();
    }
}
