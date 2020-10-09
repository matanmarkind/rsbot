// Each pixel is represented by 4 u8's, BGRA/RGBA. Each frame is a list of u8's.
pub const RAW_PIXEL_SIZE: usize = 4;

pub mod locations {
    use util::*;

    /// Absolute values describing the screen's. Most values should be given in
    /// reference to this point. Changing this will move every other "const fn"
    /// listed.
    pub const WINDOW_TOP_LEFT: Position = Position { x: 960, y: 26 };
    pub const WINDOW_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 960, dy: 637 };

    // Between the window for the program and the game screen there is a border. At
    // the top this has options such minimize, and on the sides and bottom it is
    // simply padding.
    pub const SIDE_BORDER_WIDTH: i32 = 4;
    pub const BOTTOM_BORDER_HEIGHT: i32 = 5;
    pub const TOP_BAR_HEIGHT: i32 = 27;

    /// All other locations are defined in terms of where they are in reference to
    /// WINDOW_TOP_LEFT.
    pub const SCREEN_TOP_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + SIDE_BORDER_WIDTH,
        y: WINDOW_TOP_LEFT.y + TOP_BAR_HEIGHT,
    };

    /// When the mouse hovers over an object, the action that would be taken by left
    /// clicking is displayed in the top left corner.
    pub const TOP_LEFT_ACTION_TEXT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 9,
        y: WINDOW_TOP_LEFT.y + 31,
    };

    /// Location used to click to make sure that the rs window is the active window.
    pub const TOP_BAR_MIDDLE: Position = Position {
        x: WINDOW_TOP_LEFT.x + WINDOW_DIMENSIONS.dx / 2,
        y: WINDOW_TOP_LEFT.y + TOP_BAR_HEIGHT / 2,
    };

    /// Box in the middle (ish) of the screen. Used to search for things closer to the
    /// player first.
    pub const NEARBY_SCREEN_TOP_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 100,
        y: WINDOW_TOP_LEFT.y + 224,
    };
    pub const NEARBY_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 650, dy: 350 };
    pub const VERY_NEARBY_SCREEN_TOP_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 250,
        y: WINDOW_TOP_LEFT.y + 224,
    };
    pub const VERY_NEARBY_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 350, dy: 250 };

    /// The area of the screen that should be open for searching for things in
    /// the world. This is when the chatbox is closed, and the inventory is
    /// open. With the minimap the right colum of the screen is basically out of
    /// commision.
    pub const OPEN_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 750, dy: 570 };

    /// Middle of the minimap is where player dot is located. Pressing here should
    /// not cause us to move.
    pub const MINIMAP_MIDDLE: Position = Position {
        x: WINDOW_TOP_LEFT.x + WINDOW_DIMENSIONS.dx - 85,
        y: WINDOW_TOP_LEFT.y + 113,
    };

    /// This is part of the gray background for the inventory icon in the bottom of
    /// the screen. Used to tell if the inventory is open or closed.
    pub const INVENTORY_ICON_BACKGROUND: Position = Position {
        x: WINDOW_TOP_LEFT.x + 632,
        y: WINDOW_TOP_LEFT.y + 606,
    };
    /// Interior bounds of the inventory itself.
    pub const INVENTORY_TOP_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 759,
        y: WINDOW_TOP_LEFT.y + 330,
    };
    pub const INVENTORY_BOTTOM_RIGHT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 948,
        y: WINDOW_TOP_LEFT.y + 590,
    };
    pub const INVENTORY_FIRST_SLOT: Position = Position {
        x: INVENTORY_TOP_LEFT.x + 10,
        y: INVENTORY_TOP_LEFT.y + 6,
    };
    pub const INVENTORY_SLOT_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 42, dy: 36 };

    /// Chat box/buttons.
    pub const ALL_CHAT_BUTTON: Position = Position {
        x: WINDOW_TOP_LEFT.x + 15,
        y: WINDOW_TOP_LEFT.y + 619,
    };
    pub const CHAT_BOX_TOP_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 10,
        y: WINDOW_TOP_LEFT.y + 471,
    };
    pub const CHAT_BOX_BOTTOM_LEFT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 5,
        y: WINDOW_TOP_LEFT.y + 604,
    };
    pub const CHAT_BOX_TOP_RIGHT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 518,
        y: WINDOW_TOP_LEFT.y + 473,
    };
    pub const CHAT_BOX_BOTTOM_RIGHT: Position = Position {
        x: WINDOW_TOP_LEFT.x + 520,
        y: WINDOW_TOP_LEFT.y + 604,
    };
}

pub mod colors {
    use crate::{FuzzyPixel, Pixel};

    pub const ACTION_WHITE: FuzzyPixel = FuzzyPixel {
        blue_min: 180,
        blue_max: 255,
        green_min: 180,
        green_max: 255,
        red_min: 180,
        red_max: 255,
    };

    pub const ACTION_BLUE: FuzzyPixel = FuzzyPixel {
        blue_min: 180,
        blue_max: 255,
        green_min: 180,
        green_max: 255,
        red_min: 0,
        red_max: 25,
    };

    pub const ACTION_YELLOW: FuzzyPixel = FuzzyPixel {
        blue_min: 0,
        blue_max: 30,
        green_min: 190,
        green_max: 235,
        red_min: 190,
        red_max: 235,
    };

    pub const PURE_RED: Pixel = Pixel {
        red: 255,
        green: 0,
        blue: 0,
    };

    pub const INVENTORY_BACKGROUND: FuzzyPixel = FuzzyPixel {
        blue_min: 37,
        blue_max: 46,
        green_min: 49,
        green_max: 57,
        red_min: 58,
        red_max: 65,
    };

    /// This is the red color of the inventory icon when the inventory is open.
    /// Corresponds to the location of INVENTORY_ICON_BACKGROUND.
    pub const INVENTORY_ICON_BACKGROUND_OPEN: FuzzyPixel = FuzzyPixel {
        blue_min: 25,
        blue_max: 35,
        green_min: 35,
        green_max: 45,
        red_min: 110,
        red_max: 130,
    };

    pub const TREE_BARK: FuzzyPixel = FuzzyPixel {
        blue_min: 40,
        blue_max: 44,
        green_min: 81,
        green_max: 85,
        red_min: 114,
        red_max: 118,
    };

    pub const OAK_BARK: FuzzyPixel = FuzzyPixel {
        blue_min: 40,
        blue_max: 44,
        green_min: 81,
        green_max: 85,
        red_min: 114,
        red_max: 118,
    };

    pub const SMALL_NET_FISHING_SPOT: FuzzyPixel = FuzzyPixel {
        blue_min: 105,
        blue_max: 115,
        green_min: 115,
        green_max: 135,
        red_min: 140,
        red_max: 155,
    };
}
