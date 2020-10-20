use screen::{
    action_letters::Letter,inventory_slot_pixels, colors, Capturer, Frame, FrameHandler, FuzzyPixel, Locations,
};
use std::thread::sleep;
use std::time::Duration;
use userinput::InputBot;
use util::*;

/// Looks for a pixel matching 'expected_pixels' in the minimap section of the
/// frame. This is done in a circle centered at minimap center and expands to
/// the given radius. If a matching pixel is found, we check in the immediate
/// vicinity for a 'check_pixel' to confirm we found what we want.
fn check_minimap_pixels(
    framehandler: &FrameHandler,
    frame: &screen::DefaultFrame,
    radius: f32,
    expected_pixels: &[FuzzyPixel],
    check_pixels: &[FuzzyPixel],
) -> Option<Position> {
    for fuzzy_pixel in expected_pixels.iter() {
        let pos = frame.find_pixel_random_polar(
            *fuzzy_pixel,
            framehandler.locations.minimap_middle(),
            radius,
        );
        if pos.is_none() {
            continue;
        }

        for check in check_pixels.iter() {
            if !frame
                .find_pixel_random_polar(
                    *check,
                    pos.unwrap(),
                    Locations::CHECK_ADJACENT_MINIMAP_PIXELS_RADIUS,
                )
                .is_none()
            {
                return pos;
            }
        }
    }
    None
}

#[derive(Clone, Copy)]
pub enum MousePress {
    None,
    Left,
    Right,
}

/// Enum to define what conditions we are waiting to be fulfilled before an
/// action is deemed complete. There are 2 parts generally.
/// - The enum variant, which describes the condition checked for. Time only
///   waits a set amount of time before returning true.
/// - The maximum amount of time to wait before giving up and returning a
///   failure status. Time explicitly waits for Duration and returns true.
///
/// Note that waiting happens after an action has been taken, and this is meant
/// to deal with delay in things like game rendering or effects such as lighting
/// a fire. This is not used to await action letters matching since the actions
/// associated with moving to a location and pressing must be taken before the
/// wait, and for action letters we want to prevent clicking until after the
/// check.
///
/// TODO: Add a generic to take a function.
#[derive(Clone)]
pub enum AwaitFrame {
    Time(Duration),
    IsBankOpen(Duration),
    IsInventoryOpen(Duration),
    IsWorldMapOpen(Duration),
    IsCloseOnMinimap(Duration, Vec<FuzzyPixel>, Vec<FuzzyPixel>),

    // Only to be used with DescribeActionForMinimap which converts this to
    // IsCloseOnMinimap. Otherwise this is the equivalent of Time.
    IsCloseOnMinimapIncomplete(Duration),
}

/// Checks the described action and returns true if the condition is met.
fn await_result(
    await_config: &AwaitFrame,
    framehandler: &FrameHandler,
    frame: &screen::DefaultFrame,
) -> bool {
    match await_config {
        AwaitFrame::Time(duration) => sleep(*duration),
        AwaitFrame::IsCloseOnMinimapIncomplete(_) => {
            dbg!("Illegal call to IsCloseOnMinimapIncomplete");
            return false;
        }
        AwaitFrame::IsBankOpen(_) => {
            if !framehandler.is_bank_open(frame) {
                return false;
            }
        }
        AwaitFrame::IsInventoryOpen(_) => {
            if !framehandler.is_inventory_open(frame) {
                return false;
            }
        }
        AwaitFrame::IsWorldMapOpen(_) => {
            if !framehandler.is_worldmap_open(frame) {
                return false;
            }
        }
        AwaitFrame::IsCloseOnMinimap(_, expected_pixels, check_pixels) => {
            if check_minimap_pixels(
                framehandler,
                frame,
                Locations::MINIMAP_SMALL_RADIUS,
                expected_pixels,
                check_pixels,
            )
            .is_none()
            {
                return false;
            }
        }
    }
    true
}

fn await_result_timeout(await_config: &AwaitFrame) -> Duration {
    match await_config {
        AwaitFrame::Time(duration) => *duration,
        AwaitFrame::IsCloseOnMinimapIncomplete(duration) => *duration,
        AwaitFrame::IsBankOpen(duration) => *duration,
        AwaitFrame::IsInventoryOpen(duration) => *duration,
        AwaitFrame::IsWorldMapOpen(duration) => *duration,
        AwaitFrame::IsCloseOnMinimap(duration, _, _) => *duration,
    }
}

/// This is the interface used to describe discreet actions that Player should
/// take, moving the mouse and pressing button. Actions are stitched together to
/// have the player perform a meaningful activity.
pub trait DescribeAction {
    /// Describes an action that the player should take.
    ///
    /// Returns a position to move the mouse to and how to click.
    /// - Returning None for position or mouse press will result in either not
    ///   moving the mouse or not pressing. So you can simply add a condition
    ///   that doesn't cause an action by returning Some(None,
    ///   MousePress::None).
    /// - Returning None from the function indicates a failure.
    ///
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

    /// Once an action is taken it can take time for the result to become
    /// visible (aka lighting a fire). So we may need to wait before taking the
    /// next action.
    ///
    /// This is called in a busy loop so delays between checks should be
    /// programmed into this function.
    ///
    /// Returns true once the action is complete.
    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool;

    /// The deadline that an action has for being complete. This is the max
    /// amount of time we will wait after performaing the action described in
    /// do_action for it to be considered true before considering this to be a
    /// failure.
    fn await_result_timeout(&self) -> Duration;
}

/// Basic unit of finding info in the open screen and then acting on it. Assumes
/// that the worldmap and chatbox are closed.
pub struct DescribeActionForOpenScreen {
    pub expected_pixels: Vec<FuzzyPixel>,
    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
}

/// Used to confirm that an action we are about to take is the correct one.
pub struct DescribeActionForActionText {
    pub action_text: Vec<(Letter, FuzzyPixel)>,
    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
}

// Find something in the inventory and possibly press it.
pub struct DescribeActionForInventory {
    pub expected_pixels: Vec<screen::InventorySlotPixels>,
    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
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
    pub await_action: AwaitFrame,
}

// Find something in the inventory and possibly press it.
pub struct DescribeActionForMinimap {
    /// The pixels we are looking for, to match against.
    pub expected_pixels: Vec<FuzzyPixel>,
    /// Any nearby pixels we want to use to confirm the match.
    pub check_pixels: Vec<FuzzyPixel>,

    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
}

impl DescribeAction for DescribeActionForMinimap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForMinimap");
        match check_minimap_pixels(
            framehandler,
            frame,
            Locations::MINIMAP_RADIUS,
            &self.expected_pixels,
            &self.check_pixels,
        ) {
            None => None,
            Some(pos) => Some((Some(pos), self.mouse_press)),
        }
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        let await_action;
        match self.await_action {
            AwaitFrame::IsCloseOnMinimapIncomplete(duration) => {
                await_action = AwaitFrame::IsCloseOnMinimap(
                    duration,
                    self.expected_pixels.clone(),
                    self.check_pixels.clone(),
                )
            }
            _ => await_action = self.await_action.clone(),
        }
        await_result(&await_action, framehandler, frame)
    }

    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeAction for DescribeActionForOpenScreen {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForOpenScreen");
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

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
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
                            Locations::MINIMAP_RADIUS,
                            angle_rads,
                        );
                        return Some((Some(minimap_pos), self.mouse_press));
                    }
                }
            }
        }
        None
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeAction for DescribeActionForActionText {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForActionText");
        if framehandler.check_action_letters(frame, &self.action_text[..]) {
            return Some((None, self.mouse_press));
        }
        None
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeAction for DescribeActionForInventory {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> Option<(Option<Position>, MousePress)> {
        println!("DescribeActionForInventory");
        for fuzzy_pixel in self.expected_pixels.iter() {
            // dbg!(fuzzy_pixel);
            match framehandler.first_matching_inventory_slot(frame, &fuzzy_pixel) {
                None => (),
                Some(slot_index) => {
                    dbg!(slot_index);
                    return Some((
                        Some(framehandler.locations.inventory_slot_middle(slot_index)),
                        self.mouse_press,
                    ));
                }
            }
        }
        None
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

/// Describes actions to be taken to consume items in the inventory. This action
/// is complete when we can no longer find an item to be consumed in the
/// inventory.
///
/// TODO: Turn this into a function of Player. I don't think we will need too
/// many Activities.
pub struct ConsumeInventoryOptions {
    /// Can taking this action result in us consuming multiple slots over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_slot_action: bool,

    /// Amount of time to wait between items disappearing from the inventory
    /// before we begin actions again.
    pub timeout: Duration,

    /// Every so often we can rest just to make sure the screen is properly set
    /// up. This is only useful for open screen actions. For things like banking
    /// it can be a hindrance.
    pub reset_period: Option<Duration>,

    /// Items that we wish to consume from the inventory.
    pub inventory_consumption_pixels: Vec<screen::InventorySlotPixels>,

    /// List of specific steps performed in order to fill the inventory with the
    /// desired good.
    pub actions: Vec<Box<dyn DescribeAction>>,
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
        self.close_worldmap();
        self.press_compass();
        // At the bottom in to give time for pressing in the middle of the map
        // to take effect.
        self.close_chatbox();

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

    pub fn press_compass(&mut self) {
        self.inputbot
            .move_near(&self.framehandler.locations.compass_icon());
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
        }

        // Click the center of the minimap since this will only move us a small
        // amount. Safest/easiest way I could think of torandomly left click.
        self.inputbot
            .move_near(&self.framehandler.locations.minimap_middle());
        self.inputbot.left_click();
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

                    let time = std::time::Instant::now();
                    while !act.await_result(&self.framehandler, &self.capturer.frame().unwrap()) {
                        if time.elapsed() > act.await_result_timeout() {
                            return false;
                        }
                        sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        true
    }

    pub fn consume_inventory(&mut self, options: &ConsumeInventoryOptions) {
        println!("player.consume_inventory");

        let mut time = std::time::Instant::now();
        let mut num_consecutive_failures = 0;
        loop {
            match options.reset_period {
                Some(_) => {
                    if time.elapsed() > Duration::from_secs(300) {
                        time = std::time::Instant::now();
                        self.reset();
                    }
                }
                _ => (),
            }

            let mut frame = self.capturer.frame().unwrap();
            let mut first_matching_inventory_slot = None;
            let mut inventory_slot_pixels = inventory_slot_pixels::empty();
            for pixels in options.inventory_consumption_pixels.iter() {
                first_matching_inventory_slot = self
                    .framehandler
                    .first_matching_inventory_slot(&frame, pixels);
                if !first_matching_inventory_slot.is_none() {
                    inventory_slot_pixels = *pixels;
                    break;
                }
            }

            if first_matching_inventory_slot.is_none() {
                println!("Inventory is consumed.");
                // frame.save("/tmp/screenshot_inventory_full.jpg");
                return;
            }

            let actions_succeeded = self.do_actions(&options.actions[..]);

            if !actions_succeeded {
                dbg!(actions_succeeded);
                num_consecutive_failures += 1;
                if num_consecutive_failures > 3 {
                    self.inputbot.pan_left(37.0);
                    num_consecutive_failures = 0;
                }
                continue;
            }
            num_consecutive_failures = 0;

            let mut waiting_time = std::time::Instant::now();
            while waiting_time.elapsed() < options.timeout {
                sleep(Duration::from_secs(1));
                frame = self.capturer.frame().unwrap();
                let matching_slot = self
                    .framehandler
                    .first_matching_inventory_slot(&frame, &inventory_slot_pixels);
                if matching_slot == first_matching_inventory_slot {
                    // Nothing new in the inventory, just keep waiting.
                    continue;
                }

                first_matching_inventory_slot = matching_slot;

                if !options.multi_slot_action || matching_slot.is_none() {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete. Or the inventory is full.
                    dbg!(matching_slot);
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                waiting_time = std::time::Instant::now();
            }
        }
    }
}
