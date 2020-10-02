#![allow(warnings)] // Don't warn for unused variables... At least for now.

use crate::types::*;

/// Screen based constants. Many of these assume that the screen is in the top
/// right quarter of the screen.
///
/// Beware that colors may change when runelight is the selected screen.

/// This is where we expect the game window to be.
pub const WINDOW_BOUND: BoundingBox =
    BoundingBox(Position { x: 960, y: 52 }, Position { x: 1920, y: 625 });

/// When the chat window is expanded it is expected to fill in this area.
pub const CHAT_BOX_BOUND: BoundingBox =
    BoundingBox(Position { x: 960, y: 500 }, Position { x: 1480, y: 625 });

/// The mini map and associated info should be within this box.
pub const MINI_MAP_BOUND: BoundingBox =
    BoundingBox(Position { x: 1700, y: 52 }, Position { x: 1920, y: 230 });

/// When the inventory/skills/etc. are expanded, they should fall within this range.
pub const INVENTORY_BOUND: BoundingBox =
    BoundingBox(Position { x: 1700, y: 350 }, Position { x: 1920, y: 625 });

/// This is the part of the game that shows the world when the item pouch
/// (adventure log, skills, etc) is closed, and excluding the chat
pub const CLEAR_SCREEN_BOUNDS: &[BoundingBox] = &[
    BoundingBox(Position { x: 960, y: 52 }, Position { x: 1700, y: 625 }),
    BoundingBox(Position { x: 1700, y: 230 }, Position { x: 1920, y: 625 }),
];

/// Position which should be inactive in the top bar of the game window. Can be
/// clicked to make sure that the game window is in focus.
pub const TOP_BAR: Position = Position { x: 1500, y: 40 };

/// Inventory button. Used to check if the inventory is open or not.
pub const INVENTORY_BUTTON: Position = Position { x: 1594, y: 628 };
pub const INVENTORY_OPEN: FuzzyPixel = FuzzyPixel {
    blue_min: 28,
    blue_max: 30,
    green_min: 37,
    green_max: 39,
    red_min: 112,
    red_max: 114,
};

/// Chat buttons. Need to check them to make sure the chat box is closed.
pub const ALL_CHAT_BUTTON: Position = Position { x: 975, y: 645 };
pub const ALL_CHAT_ON_HIGHLIGHT: FuzzyPixel = FuzzyPixel {
    blue_min: 39,
    blue_max: 42,
    green_min: 49,
    green_max: 52,
    red_min: 58,
    red_max: 61,
};

pub const CHAT_BOX_TOP_LEFT: (Position, FuzzyPixel) = (
    Position { x: 970, y: 497 },
    FuzzyPixel {
        blue_min: 114,
        blue_max: 114,
        green_min: 137,
        green_max: 137,
        red_min: 147,
        red_max: 147,
    },
);
pub const CHAT_BOX_BOTTOM_LEFT: (Position, FuzzyPixel) = (
    Position { x: 965, y: 630 },
    FuzzyPixel {
        blue_min: 147,
        blue_max: 147,
        green_min: 169,
        green_max: 169,
        red_min: 173,
        red_max: 173,
    },
);

pub const CHAT_BOX_TOP_RIGHT: (Position, FuzzyPixel) = (
    Position { x: 1478, y: 499 },
    FuzzyPixel {
        blue_min: 94,
        blue_max: 94,
        green_min: 112,
        green_max: 112,
        red_min: 119,
        red_max: 119,
    },
);

pub const CHAT_BOX_BOTTOM_RIGHT: (Position, FuzzyPixel) = (
    Position { x: 1480, y: 630 },
    FuzzyPixel {
        blue_min: 140,
        blue_max: 140,
        green_min: 154,
        green_max: 154,
        red_min: 162,
        red_max: 162,
    },
);

/// When selecting an action, the name of the object to act upon appears in the
/// top left in electric blue.
pub const OBJECT_NAME_BLUE: FuzzyPixel = FuzzyPixel {
    blue_min: 221,
    blue_max: 221,
    green_min: 221,
    green_max: 221,
    red_min: 0,
    red_max: 0,
};

pub const ACTION_DESCRIPTION_Y_MAX: i32 = 70;

pub const CHARACTER_WIDTH: i32 = 7;

pub const CHOP_DOWN_TREE_BOUNDS: BoundingBox =
    BoundingBox(Position { x: 967, y: 40 }, Position { x: 1070, y: 70 });

/// Pixels to check that are in the shape of the word Tree to confirm that object we hover over is in fact a tree for us to chop down.
pub const CHOP_DOWN_ACTION_OUTLINE: &[Position] = &[];
pub const TREE_ACTION_OUTLINE: &[Position] = &[];
