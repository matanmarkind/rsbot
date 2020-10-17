use screen::{action_letters::Letter, colors, Capturer, Frame, FrameHandler, FuzzyPixel};
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

pub const ACTIVITY_AWAIT_RESULT_PERIOD: Duration = Duration::from_secs(1);

/// An activity represents something the player should do with semantic meaning,
/// they are named for goals. As opposed to DescribeAction which is a specific
/// step. Activities are comprised of multiple Actions.
pub trait Activity {
    /// To perform an activity we pass in the player mutably. This is because an
    /// activity requires there to be some state between actions to track
    /// progress.
    fn do_activity(&self, player: &mut Player);
}

/// Describes actions to be taken to fill the inventory.
///
/// TODO: Remove. FillInvetory is a subset of ConsumeInventory passing in
/// INVENTORY_SLOT_EMPTY.
pub struct FillInventory {
    /// Can taking this action result in us receiving multiple items over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_item_action: bool,

    /// Amount of time to wait for item to appear in inventory before assuming
    /// we are done (resource exhausted, failed to reach resource, etc.)
    pub timeout: Duration,

    /// List of specific steps performed in order to fill the inventory with the
    /// desired good.
    pub actions: Vec<Box<dyn DescribeAction>>,
}

/// Describes actions to be taken to consume items in the inventory. This action
/// is complete when we can no longer find an item to be consumed in the
/// inventory.
///
/// TODO: Turn this into a function of Player. I don't think we will need too
/// many Activities.
pub struct ConsumeInventory {
    /// Can taking this action result in us receiving multiple items over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_item_action: bool,

    /// Amount of time to wait between items disappearing from the inventory
    /// before we begin actions again.
    pub timeout: Duration,

    /// Items that we wish to consume from the inventory.
    pub expected_pixels: Vec<screen::InventorySlotPixels>,

    /// List of specific steps performed in order to fill the inventory with the
    /// desired good.
    pub actions: Vec<Box<dyn DescribeAction>>,
}

impl Activity for ConsumeInventory {
    fn do_activity(&self, player: &mut Player) {
        println!("ConsumeInventory.do_activity");
        player.reset();

        let mut time = std::time::Instant::now();
        let mut num_consecutive_failures = 0;
        loop {
            if time.elapsed() > Duration::from_secs(300) {
                time = std::time::Instant::now();
                player.reset();
            }

            let mut frame = player.capturer.frame().unwrap();
            let mut first_open_inventory_slot = None;
            let mut iventory_slot_pixels = colors::INVENTORY_SLOT_EMPTY;
            for pixels in self.expected_pixels.iter() {
                first_open_inventory_slot = player
                    .framehandler
                    .first_matching_inventory_slot(&frame, pixels);
                if !first_open_inventory_slot.is_none() {
                    iventory_slot_pixels = *pixels;
                    break;
                }
            }
            if first_open_inventory_slot.is_none() {
                println!("Inventory is consumed. Goodbye.");
                // frame.save("/tmp/screenshot_inventory_full.jpg");
                return;
            }

            let failed_action = player.do_actions(&self.actions[..]);

            if failed_action {
                num_consecutive_failures += 1;
                if num_consecutive_failures > 3 {
                    player.inputbot.pan_left(37.0);
                    num_consecutive_failures = 0;
                }
                continue;
            }

            num_consecutive_failures = 0;
            let mut waiting_time = std::time::Instant::now();
            while waiting_time.elapsed() < self.timeout {
                sleep(Duration::from_secs(1));
                frame = player.capturer.frame().unwrap();
                let open_slot = player
                    .framehandler
                    .first_matching_inventory_slot(&frame, &iventory_slot_pixels);
                if open_slot == first_open_inventory_slot {
                    // Nothing new in the inventory, just keep waiting.
                    continue;
                }

                first_open_inventory_slot = open_slot;

                if !self.multi_item_action || open_slot.is_none() {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete. Or the inventory is full.
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                println!("reset timer for multi_item_action");
                waiting_time = std::time::Instant::now();
            }
        }
    }
}

impl Activity for FillInventory {
    fn do_activity(&self, player: &mut Player) {
        println!("FillInventory.do_activity");
        player.reset();

        let mut time = std::time::Instant::now();
        let mut num_consecutive_failures = 0;
        loop {
            if time.elapsed() > Duration::from_secs(300) {
                time = std::time::Instant::now();
                player.reset();
            }

            let mut frame = player.capturer.frame().unwrap();
            let mut first_open_inventory_slot =
                player.framehandler.first_open_inventory_slot(&frame);
            if first_open_inventory_slot.is_none() {
                println!("Inventory is full. Goodbye.");
                // frame.save("/tmp/screenshot_inventory_full.jpg");
                return;
            }

            let failed_action = player.do_actions(&self.actions[..]);

            if failed_action {
                num_consecutive_failures += 1;
                if num_consecutive_failures > 3 {
                    player.inputbot.pan_left(37.0);
                    num_consecutive_failures = 0;
                }
                continue;
            }

            num_consecutive_failures = 0;
            let mut waiting_time = std::time::Instant::now();
            while waiting_time.elapsed() < self.timeout {
                sleep(Duration::from_secs(1));
                frame = player.capturer.frame().unwrap();
                let open_slot = player.framehandler.first_open_inventory_slot(&frame);
                if open_slot == first_open_inventory_slot {
                    // Nothing new in the inventory, just keep waiting.
                    continue;
                }

                first_open_inventory_slot = open_slot;

                if !self.multi_item_action || open_slot.is_none() {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete. Or the inventory is full.
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                println!("reset timer for multi_item_action");
                waiting_time = std::time::Instant::now();
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum MousePress {
    None,
    Left,
    Right,
}

pub trait DescribeAction {
    /// Describes an action that the player should take.
    ///
    /// Returns a position to move the mouse to and how to click.
    /// - Returning None for position or mouse press will result in either not
    ///   moving the mouse or not pressing. So you can simply add a condition
    ///   that doesn't cause an action by returning Some(None,
    ///   MousePress::None).
    /// - Returning None from the function indicates a failure.
    // Unfortunately we cannot allow frame to be any impl Frame. This is because
    // then we cannot create objects of DescribeAction due to a trait having a
    // generic param. We cannot then turn impl Frame to dyn Frame because we
    // must pass off frame to other function from within 'describe_action',
    // which require impl Frame, meaning they need to know the stack size at
    // compile time. Since we know that when the bot is actually running we
    // don't want to do image manipulation we can require a specific frame type.
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)>;

    // Once an action is taken it can sometimes take time for the result to
    // become visible (aka lighting a fire). So we may need to wait before
    // taking the next action.
    fn await_result(&self);
}

/// Basic unit of finding info in the open screen and then acting on it. Assumes
/// that the worldmap and chatbox are closed.
pub struct DescribeActionForOpenScreen {
    pub expected_pixels: Vec<FuzzyPixel>,
    pub mouse_press: MousePress,
    pub await_result_time: Duration,
}

/// Used to confirm that an action we are about to take is the correct one.
pub struct DescribeActionForActionText {
    pub action_text: Vec<(Letter, FuzzyPixel)>,
    pub mouse_press: MousePress,
    pub await_result_time: Duration,
}

// Find something in the inventory and possibly press it.
pub struct DescribeActionForInventory {
    pub expected_pixels: Vec<screen::InventorySlotPixels>,
    pub mouse_press: MousePress,
    pub await_result_time: Duration,
}

/// Describes an action based on assessing the worldmap. Assumes the worldmap is
/// already open. Finds a location of the desired pixel on the worldmap then
/// uses the minimap to attempt to move in that direction.
pub struct DescribeActionForWorldmap {
    /// The pixels we are looking for, to match against.
    pub expected_pixels: Vec<FuzzyPixel>,
    /// Any nearby pixels we want to use to confirm the match.
    pub check_pixels: Vec<FuzzyPixel>,

    pub mouse_press: MousePress,
    pub await_result_time: Duration,
}

// Find something in the inventory and possibly press it.
pub struct DescribeActionForMinimap {
    /// The pixels we are looking for, to match against.
    pub expected_pixels: Vec<FuzzyPixel>,
    /// Any nearby pixels we want to use to confirm the match.
    pub check_pixels: Vec<FuzzyPixel>,

    pub mouse_press: MousePress,
    pub await_result_time: Duration,
}

impl DescribeAction for DescribeActionForMinimap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForMinimap.describe_action");
        // Distance from where we find 'expected_pixel' in the minimap that we
        // want to find the check_pixels.
        const CHECK_RADIUS: f32 = 6.0;

        for fuzzy_pixel in self.expected_pixels.iter() {
            let pos = frame.find_pixel_random_polar(
                *fuzzy_pixel,
                framehandler.locations.minimap_middle(),
                framehandler.locations.minimap_radius(),
            );
            if pos.is_none() {
                continue;
            }

            for check in self.check_pixels.iter() {
                if !frame
                    .find_pixel_random_polar(*check, pos.unwrap(), CHECK_RADIUS)
                    .is_none()
                {
                    return Some((pos, self.mouse_press));
                }
            }
        }
        None
    }

    fn await_result(&self) {
        sleep(self.await_result_time);
    }
}

impl DescribeAction for DescribeActionForOpenScreen {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForOpenScreen.describe_action");
        for (top_left, dimensions) in framehandler.locations.open_screen_search_boxes().iter() {
            for fuzzy_pixel in self.expected_pixels.iter() {
                let position = frame.find_pixel_random(&fuzzy_pixel, top_left, &dimensions);
                if !position.is_none() {
                    return Some((position, self.mouse_press));
                }
            }
        }
        None
    }

    fn await_result(&self) {
        sleep(self.await_result_time);
    }
}

impl DescribeAction for DescribeActionForWorldmap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForWorldmap.describe_action");
        // Distance from where we find 'expected_pixel' in the minimap that we
        // want to find the check_pixels.
        const CHECK_RADIUS: f32 = 6.0;

        if !framehandler.is_worldmap_open(frame) {
            println!("Expected worldmap to be open");
            frame.save("/tmp/DescribeActionForWorldmap_WorldmapClosed.jpg");
            assert!(false);
        }

        for (top_left, dimensions) in framehandler.locations.worldmap_map_search_boxes().iter() {
            for fuzzy_pixel in self.expected_pixels.iter() {
                let pos = frame.find_pixel_random(&fuzzy_pixel, top_left, &dimensions);
                if pos.is_none() {
                    continue;
                }

                for check in self.check_pixels.iter() {
                    if !frame
                        .find_pixel_random_polar(*check, pos.unwrap(), CHECK_RADIUS)
                        .is_none()
                    {
                        // Get the angle from our character to the goal. We will
                        // then map this to a location on the minimap to click
                        // in order to move us in that direction.
                        let angle_rads = (pos.unwrap()
                            - framehandler.locations.worldmap_map_middle())
                        .angle_rads();
                        let minimap_pos = polar_to_cartesian(
                            framehandler.locations.minimap_middle(),
                            framehandler.locations.minimap_radius(),
                            angle_rads,
                        );
                        return Some((Some(minimap_pos), self.mouse_press));
                    }
                }
            }
        }
        None
    }

    fn await_result(&self) {
        sleep(self.await_result_time);
    }
}

impl DescribeAction for DescribeActionForActionText {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForActionText.describe_action");
        if framehandler.check_action_letters(frame, &self.action_text[..]) {
            return Some((None, self.mouse_press));
        }
        None
    }

    fn await_result(&self) {
        sleep(self.await_result_time);
    }
}

impl DescribeAction for DescribeActionForInventory {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForInventory.describe_action");
        for fuzzy_pixel in self.expected_pixels.iter() {
            match framehandler.first_matching_inventory_slot(frame, &fuzzy_pixel) {
                None => (),
                Some(slot_index) => {
                    return Some((
                        Some(framehandler.locations.inventory_slot_middle(slot_index)),
                        self.mouse_press,
                    ));
                }
            }
        }
        None
    }

    fn await_result(&self) {
        sleep(self.await_result_time);
    }
}

// This is the player class that will tie together the userinput and screen
// crates and wrap them in specific usages.
pub struct Player {
    pub capturer: Capturer,

    pub framehandler: FrameHandler,

    pub inputbot: InputBot,
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
        println!("Player.reset");
        // Click on the game screen to make sure it is the active window.
        self.inputbot
            .move_near(&self.framehandler.locations.minimap_middle());
        self.inputbot.left_click();

        self.open_inventory();
        self.close_chatbox();
        self.close_worldmap();

        sleep(util::REDRAW_TIME);
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

    pub fn open_worldmap(&mut self) {
        let frame = self.capturer.frame().unwrap();
        if self.framehandler.is_worldmap_open(&frame) {
            // dbg!("frame already open");
            return;
        }
        self.inputbot
            .move_near(&self.framehandler.locations.worldmap_icon());
        self.inputbot.left_click();
    }

    pub fn close_worldmap(&mut self) {
        let frame = self.capturer.frame().unwrap();
        if !self.framehandler.is_worldmap_open(&frame) {
            // dbg!("frame already open");
            return;
        }
        self.inputbot
            .move_near(&self.framehandler.locations.worldmap_icon());
        self.inputbot.left_click();
    }

    fn close_chatbox(&mut self) {
        let mut frame = self.capturer.frame().unwrap();
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        self.inputbot
            .move_near(&self.framehandler.locations.all_chat_button());
        self.inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        frame = self.capturer.frame().unwrap();

        // If the chatbox is still open it's possible a different chat tab was
        // selected and now the ALL tab is on.
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        self.inputbot
            .move_near(&self.framehandler.locations.all_chat_button());
        self.inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        frame = self.capturer.frame().unwrap();

        // If the chatbox is still open this is likely due to an update such as
        // leveling up. This closes by left clicking most things
        if !self.framehandler.is_chatbox_open(&frame) {
            return;
        } // Click the center of the minimap since this will only move us a small
          // amount. Safest/easiest way I could think of torandomly left click.
        self.inputbot
            .move_near(&self.framehandler.locations.minimap_middle());
        self.inputbot.left_click();
    }

    /// Perform a list of actions. Returns false if failed to complete any of
    /// them.
    pub fn do_actions(&mut self, actions: &[Box<dyn DescribeAction>]) -> bool {
        for act in actions {
            match act.describe_action(&self.framehandler, &self.capturer.frame().unwrap()) {
                None => return false,
                Some((maybe_pos, mouse_press)) => {
                    if !maybe_pos.is_none() {
                        self.inputbot.move_to(&maybe_pos.unwrap());
                    }
                    match mouse_press {
                        MousePress::None => (),
                        MousePress::Left => self.inputbot.left_click(),
                        MousePress::Right => self.inputbot.right_click(),
                    }
                }
            }
        }
        true
    }

    pub fn press_inventory_slot(&mut self, slot_index: i32) {
        self.inputbot.move_near(
            &self
                .framehandler
                .locations
                .inventory_slot_middle(slot_index),
        );
        self.inputbot.left_click();
    }
}
