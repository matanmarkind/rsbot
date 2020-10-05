use crate::types::*;
use util::*;

// Each pixel is represented by 4 u8's, BGRA/RGBA. Each frame is a list of u8's.
pub const RAW_PIXEL_SIZE: usize = 4;

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

pub const ACTION_DESCRIPTION_Y_MAX: i32 = 70;

pub const CHARACTER_WIDTH: i32 = 7;

pub const CHOP_DOWN_TREE_BOUNDS: BoundingBox =
    BoundingBox(Position { x: 967, y: 40 }, Position { x: 1070, y: 70 });

/// Pixels to check that are in the shape of the word Tree to confirm that object we hover over is in fact a tree for us to chop down.
pub const CHOP_DOWN_ACTION_OUTLINE: &[Position] = &[];
pub const TREE_ACTION_OUTLINE: &[Position] = &[];
