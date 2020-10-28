use screen::{
    action_text, fuzzy_pixels, Capturer, Frame, FrameHandler,ActionText,
    FuzzyPixel, InventorySlotPixels, Locations,
};
use std::thread::sleep;
use std::time::Duration;
use userinput::InputBot;
use util::*;

/// Looks for a pixel matching 'expected_pixels' in the circle defined by
/// middle, radius. If a matching pixel is found, we check in the immediate
/// vicinity for all 'check_pixel's to confirm we found what we want.
fn check_map_pixels(
    frame: &screen::DefaultFrame,
    middle: Position,
    radius: i32,
    expected_pixels: &[FuzzyPixel],
    check_pixels: &[FuzzyPixel],
) -> Option<Position> {
    for fuzzy_pixel in expected_pixels.iter() {
        let pos = frame.find_pixel_random_polar(*fuzzy_pixel, middle, radius);
        if pos.is_none() {
            continue;
        }

        // Check that the found pixel is in the correct situation.
        let mut failed_check = false;
        for check in check_pixels.iter() {
            if frame
                .find_pixel_random_polar(
                    *check,
                    pos.unwrap(),
                    Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
                )
                .is_none()
            {
                failed_check = true;
                break;
            }
        }

        if !failed_check {
            return pos;
        }
    }
    None
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
    IsWorldMapClosed(Duration),
    IsChatboxOpen(Duration),

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
        AwaitFrame::Time(duration) => {
            sleep(*duration);
            true
        }
        AwaitFrame::IsCloseOnMinimapIncomplete(_) => {
            dbg!("Illegal call to IsCloseOnMinimapIncomplete");
            false
        }
        AwaitFrame::IsBankOpen(_) => framehandler.is_bank_open(frame),
        AwaitFrame::IsInventoryOpen(_) => framehandler.is_inventory_open(frame),
        AwaitFrame::IsWorldMapOpen(_) => framehandler.is_worldmap_open(frame),
        AwaitFrame::IsWorldMapClosed(_) => !framehandler.is_worldmap_open(frame),
        AwaitFrame::IsChatboxOpen(_) => framehandler.is_chatbox_open(frame),
        AwaitFrame::IsCloseOnMinimap(_, expected_pixels, check_pixels) => check_map_pixels(
            frame,
            framehandler.locations.minimap_middle(),
            Locations::MINIMAP_SMALL_RADIUS,
            expected_pixels,
            check_pixels,
        )
        .is_some(),
    }
}

fn await_result_timeout(await_config: &AwaitFrame) -> Duration {
    match await_config {
        AwaitFrame::Time(duration) => *duration,
        AwaitFrame::IsCloseOnMinimapIncomplete(duration) => *duration,
        AwaitFrame::IsBankOpen(duration) => *duration,
        AwaitFrame::IsInventoryOpen(duration) => *duration,
        AwaitFrame::IsWorldMapOpen(duration) => *duration,
        AwaitFrame::IsWorldMapClosed(duration) => *duration,
        AwaitFrame::IsChatboxOpen(duration) => *duration,
        AwaitFrame::IsCloseOnMinimap(duration, _, _) => *duration,
    }
}

#[derive(Clone, Copy)]
pub enum MousePress {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum MouseMove {
    None,
    ToDst(Position),
    FromCurrent(DeltaPosition),
}

type ActionDescription = Option<(MouseMove, MousePress)>;
const DO_NOTHING_ACTION_DESCRIPTION: ActionDescription = Some((MouseMove::None, MousePress::None));
const FAILURE_ACTION_DESCRIPTION: ActionDescription = None;

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
    ) -> ActionDescription;

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
    pub action_text: ActionText,
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

    /// Nearby pixels which all must be found.
    pub check_pixels: Vec<FuzzyPixel>,

    /// Arc of the worldmap to search. If None will search the entire worldmap.
    ///
    /// (min_angle_degrees, arc_angle_degrees).
    pub arc_of_interest: (f32, f32),

    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
}

// Find something in the inventory and possibly press it.
pub struct DescribeActionForMinimap {
    /// The pixels we are looking for, to match against.
    pub expected_pixels: Vec<FuzzyPixel>,

    /// Nearby pixels which all must be found.
    pub check_pixels: Vec<FuzzyPixel>,

    /// Max amount of time to spend searching 1 frame.
    pub search_time: Duration,

    pub mouse_press: MousePress,
    pub await_action: AwaitFrame,
}

// Closes the worldmap. If it's already open will do nothing.
pub struct DescribeActionCloseWorldmap {
    pub await_action: AwaitFrame,
}

// Opens the worldmap. If it's already open will do nothing.
pub struct DescribeActionOpenWorldmap {
    pub await_action: AwaitFrame,
}

// Attempts to turn on running.
pub struct DescribeActionEnableRun {
    pub await_action: AwaitFrame,
}

// Turns off running if on.
pub struct DescribeActionEnableWalk {
    pub await_action: AwaitFrame,
}

// Presses the compass which orients us so that the minimap and worldmap align
// in direction.
pub struct DescribeActionPressCompass {
    pub await_action: AwaitFrame,
}

// Presses the middle of the minimap. This can be used to make sure runescape is
// the active screen, close the chatbox for popups, or close the bank without
// other side effects.
pub struct DescribeActionPressMinimapMiddle {
    pub await_action: AwaitFrame,
}

pub struct DescribeActionBankQuantityAll {
    pub await_action: AwaitFrame,
}

pub struct DescribeActionBankQuantityOne {
    pub await_action: AwaitFrame,
}

// Which item slot int he bank do we want to withdraw from.
pub struct DescribeActionWithdrawFromBank {
    pub bank_slot_index: i32,
    pub await_action: AwaitFrame,
}

pub struct DescribeActionPressChatboxMiddle {
    pub await_action: AwaitFrame,
}

pub struct DescribeActionExplicitAction {
    pub action_description: ActionDescription,
    pub await_action: AwaitFrame,
}

impl DescribeAction for DescribeActionExplicitAction {
    fn describe_action(
        &self,
        _framehandler: &FrameHandler,
        _frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionExplicitAction");
        self.action_description
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
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
    ) -> ActionDescription {
        println!("DescribeActionForOpenScreen");
        for (top_left, dimensions) in framehandler.locations.open_screen_search_boxes().iter() {
            for fuzzy_pixel in self.expected_pixels.iter() {
                let position = frame.find_pixel_random(&fuzzy_pixel, top_left, &dimensions);
                if !position.is_none() {
                    return Some((MouseMove::ToDst(position.unwrap()), self.mouse_press));
                }
            }
        }
        FAILURE_ACTION_DESCRIPTION
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
    ) -> ActionDescription {
        println!("DescribeActionForActionText");
        if framehandler.check_action_text(frame, &self.action_text) {
            return Some((MouseMove::None, self.mouse_press));
        }
        FAILURE_ACTION_DESCRIPTION
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
    ) -> ActionDescription {
        println!("DescribeActionForInventory");
        for fuzzy_pixel in self.expected_pixels.iter() {
            // dbg!(fuzzy_pixel);
            match framehandler.first_matching_inventory_slot(frame, &fuzzy_pixel) {
                None => (),
                Some(slot_index) => {
                    return Some((
                        MouseMove::ToDst(framehandler.locations.inventory_slot_middle(slot_index)),
                        self.mouse_press,
                    ));
                }
            }
        }
        FAILURE_ACTION_DESCRIPTION
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
    ) -> ActionDescription {
        println!("DescribeActionForWorldmap");
        if !framehandler.is_worldmap_open(frame) {
            println!("Expected worldmap to be open");
            frame.save("/tmp/DescribeActionForWorldmap_WorldmapClosed.jpg");
            assert!(false);
        }

        let DeltaPosition { dx, dy } = framehandler.locations.worldmap_map_dimensions();
        let min_radius = 30;
        let worldmap_arc_iter = PositionIteratorCircularSpiral::new(
            framehandler.locations.worldmap_map_middle(),
            min_radius,
            /*d_radius=*/ std::cmp::min(dx, dy) / 2 - min_radius - 1,
            /*min_angle_degrees=*/ self.arc_of_interest.0,
            /*d_angle_degrees=*/ self.arc_of_interest.1,
            /*spacing=*/ 2,
        );

        for pos in worldmap_arc_iter {
            for fuzzy_pixel in self.expected_pixels.iter() {
                if !fuzzy_pixel.matches(&frame.get_pixel(&pos)) {
                    continue;
                }

                // Check that the found pixel is in the correct situation.
                let mut all_check_pixels_match = true;
                for check_pixel in self.check_pixels.iter() {
                    let adjacent_iter = PositionIteratorCircularSpiral::new(
                        pos,
                        /*min_radius=*/ 1,
                        /*d_radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
                        /*min_angle_degrees=*/ 0.0,
                        /*d_angle_degrees=*/ 360.0,
                        /*spacing=*/ 1,
                    );

                    let mut found_match = false;
                    for adjacent_pos in adjacent_iter {
                        if check_pixel.matches(&frame.get_pixel(&adjacent_pos)) {
                            found_match = true;
                            break;
                        }
                    }
                    if !found_match {
                        all_check_pixels_match = false;
                        break;
                    }
                }

                // If all of the check pixels matched, we're good to go.
                if all_check_pixels_match {
                    // Get the angle from our character to the goal. We will
                    // then map this to a location on the minimap to click
                    // in order to move us in that direction.
                    let angle_rads =
                        (pos - framehandler.locations.worldmap_map_middle()).angle_rads();
                    let minimap_pos = polar_to_cartesian(
                        framehandler.locations.minimap_middle(),
                        Locations::MINIMAP_RADIUS - 6,
                        angle_rads,
                    );
                    return Some((MouseMove::ToDst(minimap_pos), self.mouse_press));
                }
            }
        }

        FAILURE_ACTION_DESCRIPTION
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeAction for DescribeActionForMinimap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionForMinimap");

        let minimap_iter = PositionIteratorCircularSpiral::new(
            /*middle=*/ framehandler.locations.minimap_middle(),
            /*min_radius=*/ 1,
            /*d_radius=*/ Locations::MINIMAP_RADIUS,
            /*min_angle_degrees=*/ 0.0,
            /*d_angle_degrees=*/ 360.0,
            /*spacing=*/ 1,
        );

        for pos in minimap_iter {
            for fuzzy_pixel in self.expected_pixels.iter() {
                if !fuzzy_pixel.matches(&frame.get_pixel(&pos)) {
                    continue;
                }

                // Check that the found pixel is in the correct situation.
                let mut all_check_pixels_match = true;
                for check_pixel in self.check_pixels.iter() {
                    let adjacent_iter = PositionIteratorCircularSpiral::new(
                        /*middle=*/ pos,
                        /*min_radius=*/ 1,
                        /*d_radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
                        /*min_angle_degrees=*/ 0.0,
                        /*d_angle_degrees=*/ 360.0,
                        /*spacing=*/ 1,
                    );

                    let mut found_match = false;
                    for adjacent_pos in adjacent_iter {
                        if check_pixel.matches(&frame.get_pixel(&adjacent_pos)) {
                            found_match = true;
                            break;
                        }
                    }
                    if !found_match {
                        all_check_pixels_match = false;
                        break;
                    }
                }

                // If all of the check pixels matched, we're good to go.
                if all_check_pixels_match {
                    return Some((MouseMove::ToDst(pos), self.mouse_press));
                }
            }
        }

        FAILURE_ACTION_DESCRIPTION
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

impl DescribeActionCloseWorldmap {
    fn new() -> Box<Self> {
        // It should not take more than 5 seconds from the time we complete the
        // actions described until we sense the worldmap is closed.
        Box::new(DescribeActionCloseWorldmap {
            await_action: AwaitFrame::IsWorldMapClosed(Duration::from_secs(5)),
        })
    }
}

impl DescribeAction for DescribeActionCloseWorldmap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionCloseWorldmap");
        if !framehandler.is_worldmap_open(frame) {
            // dbg!("worldmap already open");
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos =
            util::random_position_polar(framehandler.locations.worldmap_icon(), /*radius=*/ 6);
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionOpenWorldmap {
    fn new() -> Box<Self> {
        // It should not take more than 5 seconds from the time we complete the
        // actions described until we sense the worldmap is open.
        Box::new(DescribeActionOpenWorldmap {
            await_action: AwaitFrame::IsWorldMapOpen(Duration::from_secs(5)),
        })
    }
}

impl DescribeAction for DescribeActionOpenWorldmap {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionOpenWorldmap");
        if framehandler.is_worldmap_open(frame) {
            // dbg!("worldmap already open");
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos =
            util::random_position_polar(framehandler.locations.worldmap_icon(), /*radius=*/ 6);
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionEnableRun {
    pub fn new() -> Box<Self> {
        Box::new(DescribeActionEnableRun {
            await_action: AwaitFrame::Time(Duration::from_secs(1)),
        })
    }
}

impl DescribeAction for DescribeActionEnableRun {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionEnableRun");
        let pos = framehandler.locations.run_icon();
        if frame.check_loose_pixel(&pos, &fuzzy_pixels::run_icon_on()) {
            // dbg!("worldmap already open");
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(pos, /*radius=*/ 6);
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionEnableWalk {
    pub fn new() -> Box<Self> {
        Box::new(DescribeActionEnableWalk {
            await_action: AwaitFrame::Time(Duration::from_secs(1)),
        })
    }
}

impl DescribeAction for DescribeActionEnableWalk {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionEnableWalk");
        let pos = framehandler.locations.run_icon();
        if !frame.check_loose_pixel(&pos, &fuzzy_pixels::run_icon_on()) {
            // dbg!("worldmap already open");
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(pos, /*radius=*/ 6);
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionPressCompass {
    fn new() -> Box<Self> {
        // We don't wait for anything to confirm this action.
        Box::new(DescribeActionPressCompass {
            await_action: AwaitFrame::Time(Duration::from_nanos(1)),
        })
    }
}

impl DescribeAction for DescribeActionPressCompass {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        _frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionPressCompass");
        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(
            framehandler.locations.compass_icon(),
            /*radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionPressMinimapMiddle {
    pub fn new() -> Box<Self> {
        // Wait 1s in case we moved a bit.
        Box::new(DescribeActionPressMinimapMiddle {
            await_action: AwaitFrame::Time(Duration::from_secs(1)),
        })
    }
}

impl DescribeAction for DescribeActionPressMinimapMiddle {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        _frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionPressMinimapMiddle");
        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(
            framehandler.locations.minimap_middle(),
            /*radius=*/ 2,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionBankQuantityAll {
    pub fn new() -> Box<Self> {
        // Wait 1s in case we moved a bit.
        Box::new(DescribeActionBankQuantityAll {
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        })
    }
}

impl DescribeAction for DescribeActionBankQuantityAll {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionBankQuantityAll");
        if framehandler.is_bank_quantity_all(frame) {
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(
            framehandler.locations.bank_quantity_all(),
            /*radius=*/ 4,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionBankQuantityOne {
    pub fn new() -> Box<Self> {
        // Wait 1s in case we moved a bit.
        Box::new(DescribeActionBankQuantityOne {
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        })
    }
}

impl DescribeAction for DescribeActionBankQuantityOne {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionBankQuantityOne");
        if framehandler.is_bank_quantity_one(frame) {
            return DO_NOTHING_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(
            framehandler.locations.bank_quantity_one(),
            /*radius=*/ 4,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionWithdrawFromBank {
    pub fn new(bank_slot_index: i32) -> Box<Self> {
        // Wait 1s in case we moved a bit.
        Box::new(DescribeActionWithdrawFromBank {
            bank_slot_index,
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        })
    }
}

impl DescribeAction for DescribeActionWithdrawFromBank {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        _frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionWithdrawFromBank");
        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = framehandler
            .locations
            .bank_slot_center(self.bank_slot_index);
        let pos = util::random_position_polar(
            pos,
            /*radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

impl DescribeActionPressChatboxMiddle {
    pub fn new() -> Box<Self> {
        // Wait 1s in case we moved a bit.
        Box::new(DescribeActionPressChatboxMiddle {
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        })
    }
}

impl DescribeAction for DescribeActionPressChatboxMiddle {
    fn describe_action(
        &self,
        framehandler: &FrameHandler,
        frame: &screen::DefaultFrame,
    ) -> ActionDescription {
        println!("DescribeActionPressChatboxMiddle");
        if !framehandler.is_chatbox_open(frame) {
            return FAILURE_ACTION_DESCRIPTION;
        }

        // Randomly shift the coordinates by 1 to avoid always pressing the same
        // pixel.
        let pos = util::random_position_polar(
            framehandler.locations.chatbox_middle(),
            /*radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
        );
        Some((MouseMove::ToDst(pos), MousePress::Left))
    }

    fn await_result(&self, framehandler: &FrameHandler, frame: &screen::DefaultFrame) -> bool {
        await_result(&self.await_action, framehandler, frame)
    }
    fn await_result_timeout(&self) -> Duration {
        await_result_timeout(&self.await_action)
    }
}

/// Use structs to pass complicated sets of params. The 2 main benefits are:
/// 1. Calling site can use named params, which are required for structs, but
///    not allowed for function calls.
/// 2. Can create the description in a helper function and keep 'main' for each
///    bot cleaner.

/// Describes actions to be taken to consume items in the inventory. This action
/// is complete when we can no longer find an item to be consumed in the
/// inventory.
pub struct ConsumeInventoryParams {
    /// Can taking this action result in us consuming multiple slots over time.
    /// If so, we will continue resetting the timer every time we receive an
    /// item. For example, a single click on an oak tree can result in us
    /// cutting many logs.
    pub multi_slot_action: bool,

    /// Amount of time to wait between slots disappearing from the inventory
    /// before we begin actions again.
    pub slot_consumption_waittime: Duration,

    /// Max amount of time to attempt performing 'actions' for.
    pub activity_timeout: Duration,

    /// The item that should be consumed from the inventory. We will continue
    /// attempting to perform 'actions' until either we time out
    /// (activity_timeout) or no slot can be found containing 'item_to_consume'.
    ///
    /// Note that using inventory_slot_pixels::empty is the equivalent of saying
    /// to fill the inventory by performing 'actions'.
    pub item_to_consume: screen::InventorySlotPixels,

    /// List of specific steps performed in order to fill the inventory with the
    /// desired good.
    pub actions: Vec<Box<dyn DescribeAction>>,
}

/// Describe where the player should travel to. Can also indicate a starting
/// direction to walk in.
///
/// If you only want to travel a short distance in a direction you can give a
/// small time for starting_direction and then immediately
/// DescribeActionPressMinimapMiddle.
///
/// Be aware that using starting_direction and arc_of_interest make this less
/// general, since this implies specific assumptions about where starting point
/// and destination are in reference to each other.
pub struct TravelToParams {
    /// Pixels that identify the destination well. Should not be extremely
    /// common ones like 'map_floor_gray' which will likely not find a specific
    /// enough point. These are the pixels that are searched for in the
    /// mini/worldmap.
    ///
    /// If empty we will not search the map.
    pub destination_pixels: Vec<FuzzyPixel>,
    /// Once a 'destination_pixel' is found, we must confirm that it is a
    /// reasonable target. This is done by checking that all of
    /// 'confirmation_pixels' are found nearby.
    pub confirmation_pixels: Vec<FuzzyPixel>,

    /// Arc of the worldmap to search. If None will search the entire worldmap.
    ///
    /// (min_angle_degrees, arc_angle_degrees).
    /// Recommended to use (0.0, 360.0) unless you have strong reason not to.
    pub arc_of_interest: (f32, f32),

    /// If true will attempt to set the player to run.
    pub try_to_run: bool,

    /// Player will travel in a straight line folloing these instructions. If
    /// this is not None, the player will start by walking in the direction
    /// given for the set amount of time.
    ///
    /// f32: this is the angle in degrees that the player should walk in. - 0 =
    ///     East, right - 90 = South, down - 180 = West, left - 270 = North, up
    ///     Duration: How long to walk for.
    ///
    pub starting_direction: Option<(f32, Duration)>,
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

        sleep(util::REDRAW_TIME);
        self.open_inventory();
        self.close_worldmap();
        self.press_compass();
        // At the bottom in to give time for pressing in the middle of the map
        // to take effect.
        self.close_chatbox();

        sleep(util::REDRAW_TIME);
    }

    /// Assumes runelight is the active screen.
    ///
    /// TODO: Move this to Action? In order to do that I'd have to switch from
    /// esc hotkey to pressing the icon.
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
            let action_description =
                act.describe_action(&self.framehandler, &self.capturer.frame().unwrap());
            if action_description.is_none() {
                return false;
            }

            let (mouse_move, mouse_press) = action_description.unwrap();

            match mouse_move {
                MouseMove::None => (),
                MouseMove::ToDst(pos) => self.inputbot.move_to(&pos),
                MouseMove::FromCurrent(delta) => {
                    let dst = self.inputbot.mouse_position() + delta;
                    self.inputbot.move_to(&dst);
                }
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
                sleep(util::REDRAW_TIME);
            }
        }

        true
    }

    pub fn drop_items(&mut self, items: &Vec<screen::InventorySlotPixels>) {
        self.open_inventory();

        self.inputbot.hold_shift();

        let mut actions = Vec::<Box<dyn DescribeAction>>::new();
        actions.push(Box::new(DescribeActionForInventory {
            expected_pixels: items.clone(),
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        }));
        while self.do_actions(&actions) {
            // Just in case we accidentally close it.
            self.open_inventory();
        }

        self.inputbot.release_shift();
    }

    /// Opens the bank and deposits all of the listed items.
    ///
    /// Assumes action text for opening the bank is always the same.
    ///
    /// Requires us to be near the bank, since we don't handle travel here.
    pub fn deposit_in_bank(
        &mut self,
        bank_colors: &Vec<FuzzyPixel>,
        items: &Vec<InventorySlotPixels>,
    ) {
        let mut open_bank_actions = Vec::<Box<dyn DescribeAction>>::new();
        open_bank_actions.push(Box::new(DescribeActionForOpenScreen {
            expected_pixels: bank_colors.clone(),
            mouse_press: MousePress::None,
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        }));
        open_bank_actions.push(Box::new(DescribeActionForActionText {
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::Time(Duration::from_nanos(1)),
            action_text: action_text::bank_bank_booth(),
        }));
        // Move so that hover text doesn't interfere.
        let minimap_middle_pos = util::random_position_polar(
            self.framehandler.locations.minimap_middle(),
            /*radius=*/ 10,
        );
        open_bank_actions.push(Box::new(DescribeActionExplicitAction {
            action_description: Some((MouseMove::ToDst(minimap_middle_pos), MousePress::None)),
            // It can take up to 10 seconds since we may need to walk.
            await_action: AwaitFrame::IsBankOpen(Duration::from_secs(10)),
        }));
        open_bank_actions.push(DescribeActionBankQuantityAll::new());

        while !self
            .framehandler
            .is_bank_open(&self.capturer.frame().unwrap())
        {
            self.do_actions(&open_bank_actions);
            // TODO: Add a timeout?
        }

        let mut deposit_actions = Vec::<Box<dyn DescribeAction>>::new();
        deposit_actions.push(Box::new(DescribeActionForInventory {
            expected_pixels: items.clone(),
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        }));

        while self.do_actions(&deposit_actions) {
            // TODO: Add a timeout?
        }
    }

    /// Opens the bank and deposits all of the listed items.
    ///
    /// Assumes action text for opening the bank is always the same.
    ///
    /// Requires us to be near the bank, since we don't handle travel in this
    /// function. We do not support stackable items, such as coins.
    ///
    /// We don't analyze the bank pixels, so you have to give the slot_index of
    /// the item to be withdrawn.
    ///
    /// If the last item in 'bank_slot_and_quantity' has quantity 0, we will
    /// withdraw All (aka all remaining inventory slots).
    pub fn withdraw_from_bank(
        &mut self,
        bank_colors: &Vec<FuzzyPixel>,
        bank_slot_and_quantity: &Vec<(i32, u32)>,
    ) {
        let mut open_bank_actions = Vec::<Box<dyn DescribeAction>>::new();
        open_bank_actions.push(Box::new(DescribeActionForOpenScreen {
            expected_pixels: bank_colors.clone(),
            mouse_press: MousePress::None,
            await_action: AwaitFrame::Time(util::REDRAW_TIME),
        }));
        open_bank_actions.push(Box::new(DescribeActionForActionText {
            mouse_press: MousePress::Left,
            await_action: AwaitFrame::Time(Duration::from_nanos(1)),
            action_text: action_text::bank_bank_booth(),
        }));
        // Move so that hover text doesn't interfere.
        let minimap_middle_pos = util::random_position_polar(
            self.framehandler.locations.minimap_middle(),
            /*radius=*/ 10,
        );
        open_bank_actions.push(Box::new(DescribeActionExplicitAction {
            action_description: Some((MouseMove::ToDst(minimap_middle_pos), MousePress::None)),
            // It can take up to 10 seconds since we may need to walk.
            await_action: AwaitFrame::IsBankOpen(Duration::from_secs(10)),
        }));

        let mut total_withdrawals = 0;
        let mut withdrawal_actions = Vec::<Box<dyn DescribeAction>>::new();
        if bank_slot_and_quantity.len() > 1 || bank_slot_and_quantity[0].1 != 0 {
            // Only the last item uses QuantityAll, and only if it is marked
            // with quantity 0.
            withdrawal_actions.push(DescribeActionBankQuantityOne::new());
        }

        for (i, (slot_index, quantity)) in bank_slot_and_quantity.iter().enumerate() {
            if i + 1 == bank_slot_and_quantity.len() && *quantity == 0 {
                withdrawal_actions.push(DescribeActionBankQuantityAll::new());
                withdrawal_actions.push(DescribeActionWithdrawFromBank::new(*slot_index));
                break;
            }

            for _ in 0..*quantity {
                total_withdrawals += 1;
                withdrawal_actions.push(DescribeActionWithdrawFromBank::new(*slot_index));
            }
        }
        // TODO: Remove if we support stackable items (aka coins). For that we
        // will also need custom Quantity.
        assert!(total_withdrawals <= Locations::NUM_BANK_SLOTS);

        while !self
            .framehandler
            .is_bank_open(&self.capturer.frame().unwrap())
        {
            self.do_actions(&open_bank_actions);
            // TODO: Add a timeout?
        }

        // Shouldn't be able to fail since withdrawal doesn't depend on any
        // conditions within the screen.
        assert!(self.do_actions(&withdrawal_actions))
    }

    // Main way of filling inventory.
    pub fn gather_resources() {}

    pub fn consume_inventory(&mut self, params: &ConsumeInventoryParams) -> bool {
        println!("player.consume_inventory");

        let timer = std::time::Instant::now();
        let mut reset_timer = std::time::Instant::now();
        let mut num_consecutive_failures = 0;
        loop {
            if timer.elapsed() > params.activity_timeout {
                return false;
            }

            if reset_timer.elapsed() > Duration::from_secs(300) {
                reset_timer = std::time::Instant::now();
                self.reset();
            }

            let mut frame = self.capturer.frame().unwrap();
            let mut first_matching_inventory_slot = self
                .framehandler
                .first_matching_inventory_slot(&frame, &params.item_to_consume);
            if first_matching_inventory_slot.is_none() {
                // If any of the comsumption items is missing we
                // have finished consuming the inventory.
                println!("Inventory has been consumed");
                // frame.save("/tmp/screenshot_inventory_full.jpg");
                return true;
            }

            let actions_succeeded = self.do_actions(&params.actions[..]);

            if !actions_succeeded {
                num_consecutive_failures += 1;
                if num_consecutive_failures > 3 {
                    self.inputbot.pan_left(37.0);
                    num_consecutive_failures = 0;
                }
                continue;
            }
            num_consecutive_failures = 0;

            let mut waiting_time = std::time::Instant::now();
            while waiting_time.elapsed() < params.slot_consumption_waittime {
                sleep(Duration::from_secs(1));
                frame = self.capturer.frame().unwrap();
                let matching_slot = self
                    .framehandler
                    .first_matching_inventory_slot(&frame, &params.item_to_consume);
                if matching_slot == first_matching_inventory_slot {
                    // Nothing new in the inventory, just keep waiting.
                    continue;
                }

                first_matching_inventory_slot = matching_slot;

                if !params.multi_slot_action || matching_slot.is_none() {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete. Or the inventory is full.
                    // dbg!(matching_slot);
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                waiting_time = std::time::Instant::now();
            }
        }
    }

    /// expected_pixels - The pixels we are looking for, to match against.
    /// check_pixels - Any nearby pixels we want to use to confirm the match.
    ///
    /// Assumes colors are the same on worldmap and minimap.
    ///
    /// TODO: Consider zooming the worldmap out if we can't find anything.
    ///
    /// TODO: Allow starting direction. User puts in (angle, time) and we walk
    /// in that direction for that amount of time.
    ///
    /// TODO: Add walk/run control.
    pub fn travel_to(&mut self, params: &TravelToParams) {
        let wait_time = Duration::from_secs(if params.try_to_run { 4 } else { 8 });
        let worldmap_action: Vec<Box<dyn DescribeAction>> = vec![
            DescribeActionOpenWorldmap::new(),
            Box::new(DescribeActionForWorldmap {
                expected_pixels: params.destination_pixels.clone(),
                check_pixels: params.confirmation_pixels.clone(),
                arc_of_interest: params.arc_of_interest,
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(wait_time),
            }),
        ];
        let minimap_action: Vec<Box<dyn DescribeAction>> =
            vec![Box::new(DescribeActionForMinimap {
                expected_pixels: params.destination_pixels.clone(),
                check_pixels: params.confirmation_pixels.clone(),
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::IsCloseOnMinimapIncomplete(Duration::from_secs(30)),
                search_time: util::REDRAW_TIME,
            })];

        let setup: Vec<Box<dyn DescribeAction>> = vec![
            DescribeActionPressCompass::new(),
            if params.try_to_run {
                DescribeActionEnableRun::new()
            } else {
                DescribeActionEnableWalk::new()
            },
        ];
        if !self.do_actions(&setup) {
            dbg!("failed to perform traveling setup");
        }

        if params.starting_direction.is_some() {
            let (degrees, duration) = params.starting_direction.unwrap();
            // Don't go to the edge of the minimap since the worldmap
            // juts in so we wouldn't move.
            let minimap_pos = util::random_position(
                &polar_to_cartesian(
                    self.framehandler.locations.minimap_middle(),
                    Locations::MINIMAP_RADIUS - 7,
                    util::degrees_to_radians(degrees),
                ),
                &DeltaPosition { dx: 3, dy: 3 },
            );

            let time = std::time::Instant::now();
            while time.elapsed() < duration {
                let wait_time = std::cmp::min(
                    duration
                        .checked_sub(time.elapsed())
                        .unwrap_or(Duration::from_nanos(1)),
                    wait_time,
                );

                let actions: Vec<Box<dyn DescribeAction>> =
                    vec![Box::new(DescribeActionExplicitAction {
                        action_description: Some((MouseMove::ToDst(minimap_pos), MousePress::Left)),
                        await_action: AwaitFrame::Time(wait_time),
                    })];
                self.do_actions(&actions);
            }
        }

        if params.destination_pixels.is_empty() {
            return;
        }

        while !self.do_actions(&minimap_action) {
            // Follow the worldmap until we find the target on the minimap.
            if !self.do_actions(&worldmap_action) {
                dbg!("Failed at traveling");
            }
        }

        let close_worldmap: Vec<Box<dyn DescribeAction>> = vec![DescribeActionCloseWorldmap::new()];
        if !self.do_actions(&close_worldmap) {
            dbg!("Failed to close the worldmap");
        }
    }
}
