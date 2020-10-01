#![allow(warnings)] // Don't warn for unused variables... At least for now.

use crate::types::*;

/// Screen based constants. Many of these assume that the screen is in the top
/// right quarter of the screen.

/// This is where we expect the game window to be.
pub const WINDOW_BOUND: BoundingBox =
    BoundingBox(Position { x: 960, y: 40 }, Position { x: 1920, y: 625 });

/// When the chat window is expanded it is expected to fill in this area.
pub const CHAT_BOX_BOUND: BoundingBox =
    BoundingBox(Position { x: 960, y: 500 }, Position { x: 1480, y: 625 });

/// The mini map and associated info should be within this box.
pub const MINI_MAP_BOUND: BoundingBox =
    BoundingBox(Position { x: 1700, y: 40 }, Position { x: 1920, y: 230 });

/// When the inventory/skills/etc. are expanded, they should fall within this range.
pub const INVENTORY_BOUND: BoundingBox =
    BoundingBox(Position { x: 1700, y: 350 }, Position { x: 1920, y: 625 });

/// This is the part of the game that shows the world when the item pouch
/// (adventure log, skills, etc) is closed, and excluding the chat
pub const CLEAR_SCREEN_BOUNDS: &[BoundingBox] = &[
    BoundingBox(Position { x: 960, y: 40 }, Position { x: 1700, y: 625 }),
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
