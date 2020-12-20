use screen::{
    action_text, fuzzy_pixels, ActionText, Capturer, Frame, FrameHandler, FuzzyPixel, Locations,
};
use std::collections::BTreeMap;
use std::thread::sleep;
use std::time::Duration;
use userinput::InputBot;
use util::*;

pub enum BankQuantity {
    All,
    X,
    One,
    Exact(i32),
}

fn check_map_pixels(
    frame: &screen::DefaultFrame,
    middle: Position,
    min_radius: i32,
    d_radius: i32,
    arc_of_interest: (f32, f32),
    primary_pixel: FuzzyPixel,
    check_pixels: &[FuzzyPixel],
) -> Option<Position> {
    let map_iter = PositionIteratorCircularSpiral::new(
        middle,
        min_radius,
        d_radius,
        /*min_angle_degrees=*/ arc_of_interest.0,
        /*d_angle_degrees=*/ arc_of_interest.1,
        /*spacing=*/ 1,
    );

    for pos in map_iter {
        if !primary_pixel.matches(&frame.get_pixel(&pos)) {
            continue;
        }

        // Check that the found pixel is in the correct situation.
        let mut all_check_pixels_match = true;
        for check_pixel in check_pixels.iter() {
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
            return Some(pos);
        }
    }
    None
}

#[derive(Clone, Debug, Copy)]
pub enum MouseClick {
    None,
    Left,
    Right,
}

fn click_mouse(inputbot: &mut InputBot, click: MouseClick) {
    match click {
        MouseClick::None => (),
        MouseClick::Left => inputbot.left_click(),
        MouseClick::Right => inputbot.right_click(),
    }
}

/// Wait for either a condition to be met or for a certain amount of time.
#[derive(Clone, Copy)]
pub enum AwaitCondition {
    Time(Duration),
    IsBankOpen,
    IsInventoryOpen,
    IsChatboxOpen,
    InventoryContains(screen::InventorySlotPixels),
    // Wait until the pixel at the given position stops matching the one
    // given.
    PixelMismatch(Position, FuzzyPixel),
    PixelMatch(Position, FuzzyPixel),
}

fn is_condition_met(
    framehandler: &mut FrameHandler,
    capturer: &mut Capturer,
    condition: AwaitCondition,
) -> bool {
    let frame = capturer.frame().unwrap();
    match condition {
        AwaitCondition::Time(duration) => {
            sleep(duration);
            true
        }
        AwaitCondition::IsBankOpen => framehandler.is_bank_open(&frame),
        AwaitCondition::IsInventoryOpen => framehandler.is_inventory_open(&frame),
        AwaitCondition::IsChatboxOpen => framehandler.is_chatbox_open(&frame),
        AwaitCondition::InventoryContains(item) => framehandler
            .first_matching_inventory_slot(&frame, &item)
            .is_some(),
        AwaitCondition::PixelMismatch(pos, pixel) => !pixel.matches(&frame.get_pixel(&pos)),
        AwaitCondition::PixelMatch(pos, pixel) => pixel.matches(&frame.get_pixel(&pos)),
    }
}

/// This trait is used to define the interface that controls how the bot will
/// behave. In it we pass all of the primitives needed for the bot to interact
/// with and understand the game.
///
/// Actions are meant to be composable so that we can build pieces out of them,
/// and then create a higher level activity that fulfills this trait by calling
/// to other actions.
///
/// We separate actions colloqially into into modules, solely for organization:
///
/// - basic_action - do not call to any other Action.
/// - compound_action - call to concrete Actions.
/// - abstract_action - call to variable Actions set per use case, making use of
///   dyn Action.
pub trait Action {
    /// Perform an action controlling player, which is an interface to the
    /// screen, keyboard, and mouse.
    ///
    /// Note that calling do_action will invalidate a frame that was retrieved
    /// before this call, which makes logical sense since we expect do_action to
    /// change the state of the game.
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool;
}

/// Click on the location for platelegs in the smithing menu.
pub struct PressSmithingPlatelegs {}

/// Useful reset. Will press near the center of the minimap.
pub struct PressMinimapMiddle {}

/// Pressing the compass resets our North South orientation and our up/down
/// orientation.
pub struct PressCompass {}

/// This assumes that the bank is closed.
pub struct OpenInventory {}

/// This is needed for open screen actions.
pub struct CloseChatbox {}

pub struct ClickKey {
    pub key: userinput::Key,
}

/// Make sure the player is in walking or running mode.
pub struct MaybeToggleRunning {
    // If true, will attempt to set the player to run.
    pub try_to_run: bool,
}

/// Make sure the worldmap is open/closed.
pub struct MaybeToggleWorldmap {
    // If true, will make sure the worldmap is open. If false will make sure
    // the worldmap is closed.
    pub worldmap_should_be_open: bool,
}

/// Use the minimap to identify a destination and walk towards it.
///
/// If this fails the most likely culprit is that one of the colors on the
/// minimap (other than an icon) changed shade; they get lighter or darker
/// during the day. It is also possible that there is an occlusion (anitem
/// on the ground creates a red dot covering part of an icon).
pub struct TravelToOnMinimap {
    /// The most identifying pixel for the destination we want to reach. This is
    /// an optimization so that we don't have to check every N pixels at every
    /// position. Since we already enforce finding all 'check_pixels' this
    /// doesn't change the logical performance of this action.
    pub primary_pixel: FuzzyPixel,

    /// Pixels other than those in 'primary_pixel' that are expected to be
    /// found adjacent to our destination. These are less identifying things
    /// like color of the floor, since many pixels will have this which aren't
    /// near the destination. These must all be found in close proximity to the
    /// destination.
    pub check_pixels: Vec<FuzzyPixel>,

    /// Arc of the map to search.
    ///
    /// (min_angle_degrees, arc_angle_degrees).
    /// Recommended to use (0.0, 360.0) unless you have strong reason not to.
    pub arc_of_interest: (f32, f32),
}

/// Use the worldmap to identify a destination and walk towards it. This only
/// moves us 1 click, since it is meant to be used in concert with the minimap
/// after each step.
///
/// These colors are expected to match those of the worldmap (not there will be
/// some differences since the minimap changes shading throughout the day).
///
/// We assume the player is always at the center of the worldmap, and when we
/// find the destination we use the minimap to walk in that direction. We don't
/// search adjacent to the player so as not ot repeat the minimap search.
pub struct TravelTowardsOnWorldmap {
    /// See fields in TravelToOnMinimap.
    pub primary_pixel: FuzzyPixel,

    pub check_pixels: Vec<FuzzyPixel>,

    pub arc_of_interest: (f32, f32),
}

/// Interact with an item in the inventory based on its appearance.
///
/// We will move the mouse to hover over it, and click according to the config.
pub struct InventorySlotAction {
    /// The item we want to click on.
    pub item: screen::InventorySlotPixels,

    /// If we successfully find and move to a slot holding 'item' how should
    /// we click the mouse.
    pub mouse_click: MouseClick,

    /// Whether or not to hold shift while clicking on the item. This allows
    /// us to perform actions with items in the inventory other than the
    /// osrs default.
    /// (https://github.com/runelite/runelite/wiki/Menu-Entry-Swapper)
    pub shift_click: bool,
}

/// Confirm that the text on the top left of the game screen describes the
/// action that we expect.
pub struct CheckActionText {
    pub action_text: ActionText,

    /// If the action text matches, we may press a mouse button.
    pub mouse_click: MouseClick,
}

/// Set the desired quantity for bank deposit/withdrawal.
///
/// Assumes the bank is open.
pub struct SetBankQuantity {
    pub quantity: BankQuantity,
}

pub struct ClickBankSlot {
    pub slot_index: i32,
    pub mouse_click: MouseClick,
}

pub struct DepositEntireInventoryToBank {
    pub open_bank_action: OpenBank,
}

/// Move to the middle of the chatbox, wait until chatbox is open, press.
pub struct ClickChatboxMiddle {
    pub await_chatbox_open: Await,
}

/// Make sure the bank is closed. Don't need to use this if you know that
/// you will take a move that doesn't depend on the bank such as TravelTo
/// since the minimap is unoccluded and the worldmap will open over the
/// bank.
pub struct CloseBank {}

pub struct Await {
    // timeout is meaningless if the condition is Time which carries its own
    // duration.
    pub condition: AwaitCondition,
    pub timeout: Duration,
}
pub struct AwaitAll {
    pub conditions: Vec<AwaitCondition>,
    pub timeout: Duration,
}

// Do not use AwaitAny with Time.
pub struct AwaitAny {
    pub conditions: Vec<AwaitCondition>,
    pub timeout: Duration,
}

impl Action for Await {
    fn do_action(
        &self,
        _inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        let time = std::time::Instant::now();
        loop {
            if is_condition_met(framehandler, capturer, self.condition) {
                return true;
            }

            if time.elapsed() > self.timeout {
                break;
            }

            sleep(Duration::from_millis(100));
        }

        false
    }
}

impl Action for AwaitAny {
    fn do_action(
        &self,
        _inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        let time = std::time::Instant::now();
        loop {
            if self
                .conditions
                .iter()
                .any(|&cond| is_condition_met(framehandler, capturer, cond))
            {
                return true;
            }

            if time.elapsed() > self.timeout {
                break;
            }

            sleep(Duration::from_millis(100));
        }

        false
    }
}

impl Action for AwaitAll {
    fn do_action(
        &self,
        _inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        let time = std::time::Instant::now();
        loop {
            if self
                .conditions
                .iter()
                .all(|&cond| is_condition_met(framehandler, capturer, cond))
            {
                return true;
            }

            if time.elapsed() > self.timeout {
                break;
            }

            sleep(Duration::from_millis(100));
        }

        false
    }
}

impl Action for PressSmithingPlatelegs {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        _capturer: &mut Capturer,
    ) -> bool {
        println!("PressSmithingPlatelegs");
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.smith_box_platelegs(),
            10,
        ));
        inputbot.left_click();
        true
    }
}

impl Action for PressMinimapMiddle {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        _capturer: &mut Capturer,
    ) -> bool {
        println!("PressMinimapMiddle");
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.minimap_middle(),
            2,
        ));
        inputbot.left_click();
        true
    }
}

impl Action for PressCompass {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        _capturer: &mut Capturer,
    ) -> bool {
        println!("PressCompass");
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.compass_icon(),
            8,
        ));
        inputbot.left_click();
        true
    }
}

impl Action for OpenInventory {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("OpenInventory");
        // TODO: consider enforcing bank closed.
        if framehandler.is_inventory_open(&capturer.frame().unwrap()) {
            return true;
        }

        inputbot.click_esc();
        true
    }
}

impl Action for ClickKey {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        _framehandler: &mut FrameHandler,
        _capturer: &mut Capturer,
    ) -> bool {
        println!("ClickKey");
        inputbot.click_key(self.key);
        true
    }
}

impl Action for CloseChatbox {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("CloseChatbox");
        if !framehandler.is_chatbox_open(&capturer.frame().unwrap()) {
            return true;
        }

        // Go click on the All tab
        inputbot.move_near(&framehandler.locations.all_chat_button());
        inputbot.left_click();

        let time = std::time::Instant::now();
        while time.elapsed() < Duration::from_secs(1) {
            if !framehandler.is_chatbox_open(&capturer.frame().unwrap()) {
                return true;
            }
            sleep(Duration::from_millis(100));
        }

        // If the chatbox is still open it's possible a different chat tab was
        // selected and now the ALL tab is on.
        if !framehandler.is_chatbox_open(&capturer.frame().unwrap()) {
            return true;
        }

        // Go click on the All tab
        inputbot.move_near(&framehandler.locations.all_chat_button());
        inputbot.left_click();

        let time = std::time::Instant::now();
        while time.elapsed() < Duration::from_secs(1) {
            if !framehandler.is_chatbox_open(&capturer.frame().unwrap()) {
                return true;
            }
            sleep(Duration::from_millis(100));
        }

        // Click the center of the minimap since this will only move us a small
        // amount. Safest/easiest way I could think of torandomly left click.
        inputbot.move_near(&framehandler.locations.minimap_middle());
        inputbot.left_click();

        let time = std::time::Instant::now();
        while time.elapsed() < Duration::from_secs(1) {
            if !framehandler.is_chatbox_open(&capturer.frame().unwrap()) {
                return true;
            }
            sleep(Duration::from_millis(100));
        }

        false
    }
}

impl MaybeToggleRunning {
    pub fn run() -> MaybeToggleRunning {
        MaybeToggleRunning { try_to_run: true }
    }
    pub fn walk() -> MaybeToggleRunning {
        MaybeToggleRunning { try_to_run: false }
    }
}

impl Action for MaybeToggleRunning {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("MaybeToggleRunning [ try_to_run: {}]", self.try_to_run);
        let mouse_pos = inputbot.mouse_position();
        let minimap_middle = framehandler.locations.minimap_middle();
        let over_minimap = (mouse_pos - minimap_middle).distance() <= Locations::MINIMAP_RADIUS;
        // The minimap is the top right of the screen, so anything higher
        // and to the right of this is assumed to be in the minimap plus
        // region. This may make us a bit oversensitive to moving the mouse
        // in case the game is in the middle of the computer screen and the
        // mouse is on another app, but this may cause surprising hover text
        // anyways, so best to play it safe.
        let minimap_plus_bottom_left = Locations::to_bottom_left(
            framehandler.locations.minimap_plus_top_left(),
            framehandler.locations.minimap_plus_dimensions(),
        );
        let over_minimap_plus =
            mouse_pos.x >= minimap_plus_bottom_left.x && mouse_pos.y <= minimap_plus_bottom_left.y;
        if over_minimap_plus && !over_minimap {
            // The mouse is in the minimap plus region, which can cause
            // highlights and hover text to mess with the boot color. If we
            // are actually over the minimap this won't cause those
            // problems, so no need to move.
            inputbot.move_to(&util::random_position_polar(
                framehandler.locations.minimap_middle(),
                Locations::MINIMAP_RADIUS,
            ));
        }

        let frame = capturer.frame().unwrap();
        let pos = framehandler.locations.run_icon();
        let is_run_on = frame.check_loose_pixel(&pos, &fuzzy_pixels::run_icon_on());
        if is_run_on == self.try_to_run {
            return true;
        }

        inputbot.move_to(&util::random_position_polar(pos, 4));
        inputbot.left_click();
        true
    }
}

impl MaybeToggleWorldmap {
    pub fn open_worldmap() -> MaybeToggleWorldmap {
        MaybeToggleWorldmap {
            worldmap_should_be_open: true,
        }
    }
    pub fn close_worldmap() -> MaybeToggleWorldmap {
        MaybeToggleWorldmap {
            worldmap_should_be_open: false,
        }
    }
}

impl Action for MaybeToggleWorldmap {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        let mouse_pos = inputbot.mouse_position();
        if (mouse_pos - framehandler.locations.worldmap_icon()).distance()
            <= (Locations::WROLDMAP_ICON_RADIUS + 2)
        {
            // The mouse is hovering over the worldmap icon, changing it's
            // color. AFAICT no hover text occludes the worldmap so this
            // should be the only problematic position.
            inputbot.move_to(&util::random_position_polar(
                framehandler.locations.minimap_middle(),
                Locations::MINIMAP_RADIUS,
            ));
        }

        let is_worldmap_open = framehandler.is_worldmap_open(&capturer.frame().unwrap());
        if is_worldmap_open == self.worldmap_should_be_open {
            return true;
        }

        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.worldmap_icon(),
            Locations::WROLDMAP_ICON_RADIUS - 3,
        ));
        inputbot.left_click();

        // Move to the minimap center. This should take enough time that the
        // worldmap opens. This also puts us on the minimap which is useful
        // for when are navigating on the worldmap.
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.minimap_middle(),
            Locations::MINIMAP_RADIUS,
        ));
        let time = std::time::Instant::now();
        while time.elapsed() < Duration::from_secs(3) {
            // Sometimes it takes a long time for the worldmap to open. Wait
            // for this.
            let is_worldmap_open = framehandler.is_worldmap_open(&capturer.frame().unwrap());
            if is_worldmap_open == self.worldmap_should_be_open {
                return true;
            }
        }
        false
    }
}

impl Action for TravelToOnMinimap {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("TravelToOnMinimap");

        // Try twice. We may find the destination but hit something in the way.
        for _ in 0..2 {
            // Find the destination on the minimap.
            match check_map_pixels(
                &capturer.frame().unwrap(),
                framehandler.locations.minimap_middle(),
                /*min_radius=*/ 1,
                /*d_radius=*/ Locations::MINIMAP_RADIUS,
                self.arc_of_interest,
                self.primary_pixel,
                &self.check_pixels,
            ) {
                None => return false, // Failed to find the dst.
                Some(pos) => inputbot.move_to(&pos),
            };

            // We are often walking/running when we try to find a
            // destination on the minimap. Since the mouse is normally the
            // slowest part, the mouse location may now be incorrect since
            // we kept moving. Move the mouse again to be closer, this
            // should be fast since we are already very close.
            match check_map_pixels(
                &capturer.frame().unwrap(),
                framehandler.locations.minimap_middle(),
                /*min_radius=*/ 1,
                /*d_radius=*/ Locations::MINIMAP_RADIUS,
                self.arc_of_interest,
                self.primary_pixel,
                &self.check_pixels,
            ) {
                None => return false, // Failed to find the dst.
                Some(pos) => inputbot.move_to(&pos),
            };
            inputbot.left_click();

            // TODO: Allow a second press on the minimap. Sometimes we are off if running.

            // Wait until we are nearby or timeout.
            let time = std::time::Instant::now();
            while time.elapsed() < Duration::from_secs(15) {
                match check_map_pixels(
                    &capturer.frame().unwrap(),
                    framehandler.locations.minimap_middle(),
                    /*min_radius=*/ 1,
                    /*d_radius=*/ Locations::MINIMAP_SMALL_RADIUS,
                    /*arc_of_interest=*/ (0.0, 360.0),
                    self.primary_pixel,
                    &self.check_pixels,
                ) {
                    None => (),
                    Some(_) => return true,
                };

                sleep(Duration::from_millis(100));
            }
        }

        false
    }
}

impl Action for TravelTowardsOnWorldmap {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("TravelTowardsOnWorldmap");
        let frame = &capturer.frame().unwrap();

        // Find the destination on the worldmap.
        let DeltaPosition { dx, dy } = framehandler.locations.worldmap_map_dimensions();
        let min_radius = 30;
        let worldmap_pos = match check_map_pixels(
            &frame,
            framehandler.locations.worldmap_map_middle(),
            min_radius,
            /*d_radius=*/ std::cmp::min(dx, dy) / 2 - min_radius - 1,
            self.arc_of_interest,
            self.primary_pixel,
            &self.check_pixels,
        ) {
            None => return false, // Failed to find the dst.
            Some(pos) => pos,
        };

        // Now that we have found the destination on the worldmap, we need to
        // translate this to a location on the minimap to press to walk in that
        // direction.
        let angle_rads = (worldmap_pos - framehandler.locations.worldmap_map_middle()).angle_rads();
        let minimap_pos = polar_to_cartesian(
            framehandler.locations.minimap_middle(),
            Locations::MINIMAP_RADIUS - 3,
            angle_rads,
        );
        inputbot.move_to(&minimap_pos);
        inputbot.left_click();

        let running = frame.check_loose_pixel(
            &framehandler.locations.run_icon(),
            &fuzzy_pixels::run_icon_on(),
        );
        sleep(Duration::from_secs(if running { 3 } else { 6 }));
        true
    }
}

impl InventorySlotAction {
    pub fn new(item: screen::InventorySlotPixels) -> InventorySlotAction {
        InventorySlotAction {
            item,
            mouse_click: MouseClick::Left,
            shift_click: false,
        }
    }
}

impl Action for InventorySlotAction {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("InventorySlotAction");
        let slot_index =
            framehandler.first_matching_inventory_slot(&capturer.frame().unwrap(), &self.item);
        if slot_index.is_none() {
            // 'item' wasn't found in the inventory.
            return false;
        }

        let slot_index = slot_index.unwrap();
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.inventory_slot_middle(slot_index),
            10,
        ));

        if self.shift_click {
            inputbot.hold_shift();
            sleep(Duration::from_millis(100));
        }
        click_mouse(inputbot, self.mouse_click);
        if self.shift_click {
            inputbot.release_shift();
        }

        true
    }
}

impl Action for CheckActionText {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("CheckActionText");
        if !framehandler.check_action_text(&capturer.frame().unwrap(), &self.action_text) {
            return false;
        }

        click_mouse(inputbot, self.mouse_click);
        true
    }
}

impl SetBankQuantity {
    fn is_bank_quantity(&self, framehandler: &mut FrameHandler, capturer: &mut Capturer) -> bool {
        let frame = capturer.frame().unwrap();
        match self.quantity {
            BankQuantity::All => framehandler.is_bank_quantity_all(&frame),
            BankQuantity::X => framehandler.is_bank_quantity_x(&frame),
            BankQuantity::One => framehandler.is_bank_quantity_one(&frame),
            BankQuantity::Exact(_) => panic!("Invalid quantity for SetBankQuantity."),
        }
    }

    fn quantity_position(&self, framehandler: &mut FrameHandler) -> Position {
        match self.quantity {
            BankQuantity::All => framehandler.locations.bank_quantity_all(),
            BankQuantity::X => framehandler.locations.bank_quantity_x(),
            BankQuantity::One => framehandler.locations.bank_quantity_one(),
            BankQuantity::Exact(_) => panic!("Invalid quantity for SetBankQuantity."),
        }
    }
}

impl Action for SetBankQuantity {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("SetBankQuantity");
        let top_left = framehandler.locations.bank_top_left();
        let bottom_right = Locations::to_bottom_right(
            framehandler.locations.bank_top_left(),
            framehandler.locations.bank_dimensions(),
        );
        let mouse_pos = inputbot.mouse_position();
        if (mouse_pos.x > top_left.x && mouse_pos.y > top_left.y)
            && (mouse_pos.x < top_left.x && mouse_pos.y < top_left.y)
        {
            // The mouse is hovering over the bank. This can result in hover
            // text appearing which would obscure the quantity buttons. We
            // move to the bottom right corner as a compromise between being
            // near the quantity buttons if we need to press them, near the
            // bank slots for withdrawal, or near the inventory for deposit.
            inputbot.move_to(&bottom_right);
        }

        if self.is_bank_quantity(framehandler, capturer) {
            return true;
        }

        inputbot.move_to(&util::random_position_polar(
            self.quantity_position(framehandler),
            /*radius=*/ 4,
        ));
        inputbot.left_click();
        true
    }
}

impl Action for ClickBankSlot {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        _capturer: &mut Capturer,
    ) -> bool {
        println!("ClickBankSlot");

        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.bank_slot_center(self.slot_index),
            10,
        ));
        click_mouse(inputbot, self.mouse_click);

        true
    }
}

impl DepositEntireInventoryToBank {
    pub fn new(bank_pixels: Vec<FuzzyPixel>) -> DepositEntireInventoryToBank {
        DepositEntireInventoryToBank {
            open_bank_action: OpenBank::new(
                /*expected_pixels=*/ bank_pixels,
                /*timeout=*/ Duration::from_secs(60),
            ),
        }
    }
}

impl Action for DepositEntireInventoryToBank {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("DepositEntireInventoryToBank");
        if !self
            .open_bank_action
            .do_action(inputbot, framehandler, capturer)
        {
            println!("--- Unable to open the bank ---");
            return false;
        }

        inputbot.move_to(&framehandler.locations.bank_deposit_inventory());
        sleep(Duration::from_millis(100));
        inputbot.left_click();

        // For safety press again since sometimes the bank isn't responsive.
        sleep(Duration::from_millis(100));
        inputbot.left_click();

        true
    }
}

impl ClickChatboxMiddle {
    pub fn new() -> ClickChatboxMiddle {
        ClickChatboxMiddle {
            await_chatbox_open: Await {
                condition: AwaitCondition::IsChatboxOpen,
                timeout: Duration::from_secs(3),
            },
        }
    }
}

impl Action for ClickChatboxMiddle {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("ClickChatboxMiddle");
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.chatbox_middle(),
            20,
        ));

        if self
            .await_chatbox_open
            .do_action(inputbot, framehandler, capturer)
        {
            // Sleep here so we don't have perfect reflexes.
            sleep(Duration::from_millis(100));
            inputbot.left_click();
            return true;
        }
        false
    }
}

impl Action for CloseBank {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("CloseBank");
        // Don't start by checking if the bank is open since hover text
        // could interfere.
        inputbot.move_to(&util::random_position_polar(
            framehandler.locations.minimap_middle(),
            2,
        ));
        inputbot.left_click();

        let await_time = std::time::Instant::now();
        while await_time.elapsed() < Duration::from_secs(5) {
            if !framehandler.is_bank_open(&capturer.frame().unwrap()) {
                return true;
            }
            sleep(Duration::from_millis(100));
        }

        false
    }
}

/// Have the player walk in a straight line, clicking on the minimap to walk.
/// Since this is timed and we don't measure where we are, we only walk. This is
/// to avoid expecting us to run and then being unable to and being thrown off.
///
/// We recommend relying on this as little as possible, and preferring TravelTo,
/// since that will find a destination on the map and is better at correcting
/// for errors.
///
/// Cannot fail.
pub struct TravelStraight {
    /// Direction that the player should move in in degrees.
    ///
    /// - 0 = East, right
    /// - 90 = South, down
    /// - 180 = West, left
    /// - 270 = North, up
    pub direction_degrees: f32,

    /// Approximate amount of time the player should walk for if. Note that we
    /// will not enforce stopping so it is possible for the player to keep
    /// walking until the last spot clicked on the minimap after this returns.
    pub travel_time: Duration,
}

/// Combines usage of the minimap and worldmap to make the player run/walk to a
/// destination.
pub struct TravelTo {
    pub travel_minimap: TravelToOnMinimap,
    pub travel_worldmap: TravelTowardsOnWorldmap,
    pub try_to_run: bool,
    pub timeout: Duration,
}

/// Used to find a matching pixel on the open screen and potentially click
/// on it.
///
/// Assumes that the chatbox, bank, and worldmap are all closed. This
/// doens't look at the right column of the screen with the inventory and
/// minimap_plus.
pub struct OpenScreenAction {
    /// These are the pixels of interest that we want to move the mouse to.
    pub expected_pixels: Vec<FuzzyPixel>,

    /// There may be an action that we want to confirm before taking it.
    pub check_action_text: Option<CheckActionText>,

    /// How to press the mouse.
    pub mouse_click: MouseClick,
}

/// Assumes we are near the bank and there is nothing in the way like a
/// closed door.
pub struct OpenBank {
    pub action: OpenScreenAction,

    pub timeout: Duration,
}

/// Deposit all matching items into the bank.
///
/// Assumes we are near the bank and there is nothing in the way like a
/// closed door.
pub struct DepositInBank {
    pub open_bank_action: OpenBank,
    pub quantity_all_action: SetBankQuantity,
    pub deposit_actions: Vec<InventorySlotAction>,
}

impl TravelStraight {
    /// Where on the minimap to click.
    pub fn get_minimap_pos(&self, framehandler: &FrameHandler) -> Position {
        // Don't go to the edge of the minimap since the worldmap
        // juts in so we wouldn't move.
        let minimap_radius = Locations::MINIMAP_RADIUS - 7;
        // Choose a random location on the minimap, near the edge, in the
        // direction we want to go.
        let minimap_pos = util::random_position(
            &polar_to_cartesian(
                framehandler.locations.minimap_middle(),
                minimap_radius,
                util::degrees_to_radians(self.direction_degrees),
            ),
            &DeltaPosition { dx: 3, dy: 3 },
        );
        minimap_pos
    }
}

impl Action for TravelStraight {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        // Make sure that we are walking for the timing to be accurate.
        let enable_walking = MaybeToggleRunning::walk();
        enable_walking.do_action(inputbot, framehandler, capturer);

        // Get the location on the minimap that we will press repeatedly to move
        // in a straight line.
        let minimap_pos = self.get_minimap_pos(framehandler);

        // Continually press on this spot until we are done.
        let time = std::time::Instant::now();
        while time.elapsed() < self.travel_time {
            let wait_time = std::cmp::min(
                self.travel_time
                    .checked_sub(time.elapsed())
                    .unwrap_or(Duration::from_nanos(1)),
                Duration::from_secs(8),
            );
            inputbot.move_to(&minimap_pos);
            inputbot.left_click();
            sleep(wait_time);
        }

        true
    }
}

impl TravelTo {
    pub fn new(
        primary_pixel: FuzzyPixel,
        check_pixels: Vec<FuzzyPixel>,
        arc_of_interest: (f32, f32),
        timeout: Duration,
        try_to_run: bool,
    ) -> TravelTo {
        TravelTo {
            travel_minimap: TravelToOnMinimap {
                primary_pixel,
                check_pixels: check_pixels.clone(),
                arc_of_interest,
            },
            travel_worldmap: TravelTowardsOnWorldmap {
                primary_pixel,
                check_pixels,
                arc_of_interest,
            },
            timeout,
            try_to_run,
        }
    }
}

impl Action for TravelTo {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("TravelTo");
        if self.try_to_run {
            MaybeToggleRunning::run().do_action(inputbot, framehandler, capturer);
        } else {
            MaybeToggleRunning::walk().do_action(inputbot, framehandler, capturer);
        }

        let mut worldmap_search_failed = false;
        let mut is_worldmap_open = false;
        let time = std::time::Instant::now();
        while time.elapsed() < self.timeout {
            if self
                .travel_minimap
                .do_action(inputbot, framehandler, capturer)
            {
                // We should be at the destination.
                if is_worldmap_open {
                    MaybeToggleWorldmap::close_worldmap().do_action(
                        inputbot,
                        framehandler,
                        capturer,
                    );
                } else {
                    // Once we are nearby we often will still move for another few
                    // seconds. This can cause us to click on an incorrect spot. So wait
                    // to make sure we are done moving.
                    let running = capturer.frame().unwrap().check_loose_pixel(
                        &framehandler.locations.run_icon(),
                        &fuzzy_pixels::run_icon_on(),
                    );
                    sleep(Duration::from_secs(if running { 2 } else { 4 }));
                }

                return true;
            } else if worldmap_search_failed {
                // First check the minimap, then the worldmap. It is possible during
                // the delay between searching the minimap to searching the worldmap
                // that the destination will cross the boundary. Therefore if
                // worldmap search fails, give minimap search a second chance.
                break;
            }

            // We either did not find the destination on the minimap, or we did
            // and we failed to get to it.

            if !is_worldmap_open {
                PressCompass {}.do_action(inputbot, framehandler, capturer);

                // After press compass so we are never over the worldmap at
                // this stage.
                is_worldmap_open = MaybeToggleWorldmap::open_worldmap().do_action(
                    inputbot,
                    framehandler,
                    capturer,
                );
                if !is_worldmap_open {
                    return false;
                }
            }

            worldmap_search_failed =
                !self
                    .travel_worldmap
                    .do_action(inputbot, framehandler, capturer);
        }

        if is_worldmap_open {
            MaybeToggleWorldmap::close_worldmap().do_action(inputbot, framehandler, capturer);
        }

        false
    }
}

impl OpenScreenAction {
    pub fn new(
        expected_pixels: Vec<FuzzyPixel>,
        action_text: Option<ActionText>,
        mouse_click: MouseClick,
    ) -> OpenScreenAction {
        let check_action_text = if action_text.is_none() {
            None
        } else {
            Some(CheckActionText {
                action_text: action_text.unwrap(),
                mouse_click,
            })
        };

        OpenScreenAction {
            expected_pixels,
            check_action_text,
            mouse_click,
        }
    }
}

impl Action for OpenScreenAction {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("OpenScreenAction");
        for (top_left, dimensions) in framehandler.locations.open_screen_search_boxes().iter() {
            for fuzzy_pixel in self.expected_pixels.iter() {
                let pos = capturer.frame().unwrap().find_pixel_random(
                    &fuzzy_pixel,
                    top_left,
                    &dimensions,
                );
                if pos.is_none() {
                    continue;
                }

                inputbot.move_to(&pos.unwrap());

                if self.check_action_text.is_none() {
                    click_mouse(inputbot, self.mouse_click);
                    return true;
                }

                // CheckActionText includes clicking.
                let action_text_time = std::time::Instant::now();
                while action_text_time.elapsed() < util::REDRAW_TIME {
                    // Put the sleep first to lower the chance of us reading
                    // an old action text.

                    sleep(Duration::from_millis(100));
                    if self.check_action_text.as_ref().unwrap().do_action(
                        inputbot,
                        framehandler,
                        capturer,
                    ) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl OpenBank {
    pub fn new(bank_pixels: Vec<FuzzyPixel>, timeout: Duration) -> OpenBank {
        OpenBank {
            action: OpenScreenAction::new(
                /*expected_pixels=*/ bank_pixels,
                /*action_text=*/ Some(action_text::bank_bank_booth()),
                /*mouse_click=*/ MouseClick::Left,
            ),
            timeout,
        }
    }
}

impl Action for OpenBank {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("OpenBank");
        if framehandler.is_bank_open(&capturer.frame().unwrap()) {
            return true;
        }

        let time = std::time::Instant::now();
        while time.elapsed() < self.timeout {
            if !self.action.do_action(inputbot, framehandler, capturer) {
                // We were unabe to find a matching spot. Pan the screen in
                // case the angle is a problem.
                inputbot.pan_left(37.0);
                continue;
            }

            // Make sure hover text isn't covering the bank corners.
            let bottom_right = Locations::to_bottom_right(
                framehandler.locations.bank_top_left(),
                framehandler.locations.bank_dimensions(),
            );
            inputbot.move_to(&util::random_position(
                /*top_left=*/ &bottom_right,
                &DeltaPosition { dx: 50, dy: 50 },
            ));

            let await_time = std::time::Instant::now();
            while await_time.elapsed() < Duration::from_secs(10) {
                if framehandler.is_bank_open(&capturer.frame().unwrap()) {
                    return true;
                }
                sleep(Duration::from_millis(100));
            }
        }

        // we were unable to open the bank.
        false
    }
}

impl DepositInBank {
    pub fn new(
        bank_pixels: Vec<FuzzyPixel>,
        items: Vec<screen::InventorySlotPixels>,
    ) -> DepositInBank {
        let mut deposit_actions = Vec::<InventorySlotAction>::new();
        for item in items {
            deposit_actions.push(InventorySlotAction::new(item));
        }

        DepositInBank {
            open_bank_action: OpenBank::new(
                /*expected_pixels=*/ bank_pixels,
                /*timeout=*/ Duration::from_secs(60),
            ),
            quantity_all_action: SetBankQuantity {
                quantity: BankQuantity::All,
            },
            deposit_actions,
        }
    }
}

impl Action for DepositInBank {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("DepositInBank");
        if !self
            .open_bank_action
            .do_action(inputbot, framehandler, capturer)
        {
            println!("--- Unable to open the bank ---");
            return false;
        }

        // Add a slight delay since sometimes it takes a moment for the bank
        // being open to allow us to interact with it.
        sleep(util::REDRAW_TIME);

        if !self
            .quantity_all_action
            .do_action(inputbot, framehandler, capturer)
        {
            // This should be impossible.
            println!("--- Unable to set the quantity ---");
            return false;
        }

        for action in &self.deposit_actions {
            // Don't check for success here since we pass in items which may
            // not always appear in the inventory.
            action.do_action(inputbot, framehandler, capturer);
            sleep(Duration::from_millis(100));
        }

        sleep(util::REDRAW_TIME);

        // For safety do it again since sometimes the item is not removed
        // even after pressing it. This will likely result in double
        // clicking on an item if there is only 1 of it in the inventory.
        for action in &self.deposit_actions {
            // Don't check for success here since we pass in items which may
            // not always appear in the inventory.
            action.do_action(inputbot, framehandler, capturer);
            sleep(Duration::from_millis(100));
        }

        true
    }
}

/// Perform actions which should cause the given item to be consumed. Will
/// retry if this doesn't happen.
pub struct ConsumeSingleInventoryItem {
    /// The item that should be consumed from the inventory. We will continue
    /// attempting to perform 'actions' until either we time out
    /// (activity_timeout) or no slot can be found containing 'item_to_consume'.
    ///
    /// Note that using inventory_slot_pixels::empty is the equivalent of saying
    /// to fill the inventory by performing 'actions'.
    pub item_to_consume: screen::InventorySlotPixels,

    /// List of specific steps performed in order to fill the inventory with the
    /// desired good.
    pub actions: Vec<Box<dyn Action>>,

    pub timeout: Duration,
}

/// This is used as a general framework for interacting with the game in a way
/// that will change the inventory. A common usage is gathering resources, in
/// which case you want to consume the empty slots in the inventory. Another
/// option is making pizzas in which case you want to start with the inventory
/// and use pizza on a cooking range.
///
/// The exact actions to take are defined by the user, which is why this is abstract.
pub struct ConsumeInventory {
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
    pub actions: Vec<Box<dyn Action>>,
}

pub struct ExplicitActions {
    pub actions: Vec<Box<dyn Action>>,
}

/// Withdraw the given items from the bank. This works by being given the
/// slot index in the bank of the items.
///
/// Assumes that the bank is scrolled all the way up so that the slot are in
/// their expected locations.
///
/// Assumes we are near the bank and there is nothing in the way like a
/// closed door.
///
/// TODO: Swap 0 to X and -1 to ALL in case both get used. All should always
/// be last.
pub struct WithdrawFromBank {
    pub open_bank_action: OpenBank,

    /// This is the set of actions to take once the bank is open. It should
    /// be in the pattern of [set_quantity, press_slot_center, .*].
    pub withdrawal_actions: Vec<Box<dyn Action>>,

    // Items that should be withdrawn from the bank. Used to check withdrawal
    // succeeded.
    pub await_items: AwaitAll,
}

impl Action for ConsumeSingleInventoryItem {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("ConsumeSingleInventoryItem");
        let first_matching_inventory_slot = framehandler
            .first_matching_inventory_slot(&capturer.frame().unwrap(), &self.item_to_consume);
        if first_matching_inventory_slot.is_none() {
            println!("Inventory has been consumed");
            return false;
        }

        let timer = std::time::Instant::now();
        while timer.elapsed() < self.timeout {
            for action in &self.actions {
                if !action.do_action(inputbot, framehandler, capturer) {
                    return false;
                }
            }

            let waittime = std::time::Instant::now();
            while waittime.elapsed() < Duration::from_secs(5) {
                sleep(Duration::from_millis(100));
                let matching_slot = framehandler.first_matching_inventory_slot(
                    &capturer.frame().unwrap(),
                    &self.item_to_consume,
                );
                if first_matching_inventory_slot != matching_slot {
                    return true;
                }
            }
        }
        false
    }
}

// TODO: consider adding reset.
impl Action for ConsumeInventory {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("ConsumeInventory");

        let timer = std::time::Instant::now();
        // Number of times in a row we have failed to perform the action.
        let mut consecutive_action_failures = 0;
        // Number of times in a row we succeeded in performing the action but
        // failed to consume any inventory slots.
        let mut consecutive_consumption_failures = 0;
        while timer.elapsed() < self.activity_timeout {
            let mut first_matching_inventory_slot = framehandler
                .first_matching_inventory_slot(&capturer.frame().unwrap(), &self.item_to_consume);
            if first_matching_inventory_slot.is_none() {
                println!("Inventory has been consumed");
                return true;
            }

            let mut actions_succeeded = true;
            for action in &self.actions {
                if !action.do_action(inputbot, framehandler, capturer) {
                    actions_succeeded = false;
                    break;
                }
            }

            if !actions_succeeded {
                consecutive_action_failures += 1;
                if consecutive_action_failures > 3 {
                    inputbot.pan_left(37.0);
                    consecutive_action_failures = 0;
                }
                continue;
            }
            consecutive_action_failures = 0;

            let mut waittime = std::time::Instant::now();
            let mut consumed_slot = false;
            while waittime.elapsed() < self.slot_consumption_waittime {
                sleep(Duration::from_secs(1));
                let matching_slot = framehandler.first_matching_inventory_slot(
                    &capturer.frame().unwrap(),
                    &self.item_to_consume,
                );
                if matching_slot == first_matching_inventory_slot {
                    // Nothing new in the inventory, just keep waiting.
                    continue;
                }

                consumed_slot = true;
                first_matching_inventory_slot = matching_slot;

                if !self.multi_slot_action || matching_slot.is_none() {
                    // We just received the item we were after, and we can't
                    // continue to receive, so stop waiting for the action to
                    // complete. Or the inventory is full.
                    // dbg!(matching_slot);
                    break;
                }

                // We have received an item so reset the timer to allow us to get more.
                waittime = std::time::Instant::now();
            }

            if !consumed_slot {
                // This could indicate crowding. Potentially world switch?
                consecutive_consumption_failures += 1;
                if consecutive_consumption_failures > 3 {
                    // TODO: return false?
                }
            } else {
                consecutive_consumption_failures = 0;
            }
        }

        false
    }
}

impl ExplicitActions {
    /// Default set of actions useful for resetting the player.
    pub fn default_reset() -> ExplicitActions {
        let mut actions = Vec::<Box<dyn Action>>::new();
        actions.push(Box::new(PressMinimapMiddle {}));
        actions.push(Box::new(Await {
            condition: AwaitCondition::Time(Duration::from_secs(1)),
            timeout: Duration::from_secs(0),
        }));
        actions.push(Box::new(MaybeToggleWorldmap::close_worldmap()));
        actions.push(Box::new(PressCompass {}));
        actions.push(Box::new(CloseChatbox {}));
        actions.push(Box::new(OpenInventory {}));

        ExplicitActions { actions }
    }
}

impl Action for ExplicitActions {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        for action in &self.actions {
            if !action.do_action(inputbot, framehandler, capturer) {
                return false;
            }
        }

        true
    }
}

impl WithdrawFromBank {
    /// 'bank_pixels' - pixels to look for on open screen action to open the
    /// bank.
    ///
    /// 'bank_slot_and_quantity' - The bank_slot index and the quantity to
    /// withdraw of each item.
    ///
    /// If quantity is 0, we will use All. Only 1 item can set this. If
    /// quantity is -1, we will use X. Values outside of [-1, 28] are
    /// invalid.
    ///
    /// Note that if using quantity X, this must be set before calling to
    /// this action.
    pub fn new(
        bank_pixels: Vec<FuzzyPixel>,
        bank_slot_and_quantity_and_item: Vec<(i32, BankQuantity, screen::InventorySlotPixels)>,
    ) -> WithdrawFromBank {
        // Organize the withdrawals by quantity to optimize the number of
        // times we reset. Use a BTreeMap since the special values are the
        // lowest ones, so when we reach 1-28, we won't have to keep
        // resetting quantity to One.
        let mut quantity_to_slot_indices = BTreeMap::<i32, Vec<i32>>::new();
        let mut await_conditions = Vec::<AwaitCondition>::new();
        for (slot_index, quantity, item) in bank_slot_and_quantity_and_item {
            await_conditions.push(AwaitCondition::InventoryContains(item));
            match quantity {
                BankQuantity::All => quantity_to_slot_indices
                    .entry(-1)
                    .or_insert(Vec::<i32>::new())
                    .push(slot_index),
                BankQuantity::X => quantity_to_slot_indices
                    .entry(0)
                    .or_insert(Vec::<i32>::new())
                    .push(slot_index),
                BankQuantity::Exact(val) => {
                    assert!(val <= 28);
                    quantity_to_slot_indices
                        .entry(val)
                        .or_insert(Vec::<i32>::new())
                        .push(slot_index);
                }
                BankQuantity::One => panic!("Invalid BankQuantity for WithdrawFromBank."),
            }
        }

        // Convert each item we need to withdraw into actions.
        let mut withdrawal_actions = Vec::<Box<dyn Action>>::new();
        let mut set_quantity_to_one = false;
        // Use BTreeMap + reverse iterator to erach All last.
        for (quantity, slot_indices) in quantity_to_slot_indices.iter().rev() {
            if *quantity == -1 {
                assert_eq!(slot_indices.len(), 1);

                withdrawal_actions.push(Box::new(SetBankQuantity {
                    quantity: BankQuantity::All,
                }));

                withdrawal_actions.push(Box::new(ClickBankSlot {
                    slot_index: slot_indices[0],
                    mouse_click: MouseClick::Left,
                }));
            } else if *quantity == 0 {
                withdrawal_actions.push(Box::new(SetBankQuantity {
                    quantity: BankQuantity::X,
                }));

                for slot_index in slot_indices.iter() {
                    withdrawal_actions.push(Box::new(ClickBankSlot {
                        slot_index: *slot_index,
                        mouse_click: MouseClick::Left,
                    }));
                }
            } else {
                if !set_quantity_to_one {
                    // We rely on BTreeMap to guarantee we only need to call
                    // this once.
                    withdrawal_actions.push(Box::new(SetBankQuantity {
                        quantity: BankQuantity::One,
                    }));
                    set_quantity_to_one = true;
                }

                for slot_index in slot_indices.iter() {
                    for _ in 0..*quantity {
                        withdrawal_actions.push(Box::new(ClickBankSlot {
                            slot_index: *slot_index,
                            mouse_click: MouseClick::Left,
                        }));
                    }
                }
            }
        }

        WithdrawFromBank {
            open_bank_action: OpenBank::new(
                /*expected_pixels=*/ bank_pixels,
                /*timeout=*/ Duration::from_secs(60),
            ),
            withdrawal_actions,
            await_items: AwaitAll {
                conditions: await_conditions,
                timeout: Duration::from_secs(5),
            },
        }
    }
}

impl Action for WithdrawFromBank {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        println!("WithdrawFromBank");
        if !self
            .open_bank_action
            .do_action(inputbot, framehandler, capturer)
        {
            return false;
        }

        for action in &self.withdrawal_actions {
            if !action.do_action(inputbot, framehandler, capturer) {
                return false;
            }
            sleep(Duration::from_millis(100));
        }

        self.await_items.do_action(inputbot, framehandler, capturer)
    }
}
