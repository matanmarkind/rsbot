use crate::constants::colors;
use crate::locations;
use crate::Frame;
use util::*;

// TODO: Make sure mouse isn't hovering over the inventory.
// Add IndexChecker to bot class to be responsible for this.

// Inventory slots are organize rowise. So each row is filled from left to
// right and then the next row is filled left to right.
pub const NUM_INVENTORY_ROWS: i32 = 7;
pub const NUM_INVENTORY_COLS: i32 = 4;
pub const NUM_INVENTORY_SLOTS: i32 = NUM_INVENTORY_ROWS * NUM_INVENTORY_COLS;

// Spacing between pixels to check in a slot.
const SLOT_CHECK_SPACING: DeltaPosition = DeltaPosition { dx: 7, dy: 7 };

pub fn is_inventory_open(frame: &impl Frame) -> bool {
    // Use check_loose_pixel because the background color of the icons is very
    // distinct between on and off and the satchel depicted is also a
    // significantly different color. If the image shifts, which it sometimes
    // does I don't want to be too brittle since I think the risk of a false
    // positive is relatively low.
    frame.check_loose_pixel(
        &locations::INVENTORY_ICON_BACKGROUND,
        &colors::INVENTORY_ICON_BACKGROUND_OPEN,
    )
}

fn index_to_rc(slot_index: i32) -> (i32, i32) {
    let row = slot_index / NUM_INVENTORY_COLS;
    let col = slot_index - row * NUM_INVENTORY_COLS;
    (row, col)
}

fn slot_top_left(slot_index: i32) -> Position {
    let (row, col) = index_to_rc(slot_index);
    let DeltaPosition { dx, dy } = locations::INVENTORY_SLOT_DIMENSIONS;
    Position {
        x: locations::INVENTORY_FIRST_SLOT.x + dx * col,
        y: locations::INVENTORY_FIRST_SLOT.y + dy * row,
    }
}

/// 'slot' is the 0 indexed id of this inventory slot, [0,
/// NUM_INVENTORY_SLOTS). So the first row is (0, 1, 2, 3).
// pub fn is_slot_open(frame: &impl Frame, slot_index: i32) -> bool {
pub fn is_slot_open(frame: &impl Frame, slot_index: i32) -> bool {
    let top_left = slot_top_left(slot_index);
    let past_bottom_right = &top_left + &locations::INVENTORY_SLOT_DIMENSIONS;

    // Don't bother checking the border between slots.
    let first_pos = &top_left + &SLOT_CHECK_SPACING;
    let mut pos = first_pos.clone();
    while pos.y < past_bottom_right.y {
        while pos.x < past_bottom_right.x {
            let pixel = frame.get_pixel(&pos);
            if !colors::INVENTORY_BACKGROUND.matches(&pixel) {
                // println!("is_slot_open={}, {:?}, {:?}", slot_index, pos, pixel);
                return false;
            }
            pos = Position {
                x: pos.x + SLOT_CHECK_SPACING.dx,
                y: pos.y,
            };
        }
        pos = Position {
            x: first_pos.x,
            y: pos.y + SLOT_CHECK_SPACING.dy,
        };
    }
    true
}

/// Get the minimum slot_index [0,NUM_INVENTORY_SLOTS) which points to an open
/// slot. Returns None if there is no open slot.
pub fn first_open_slot(frame: &impl Frame) -> Option<i32> {
    for i in 0..NUM_INVENTORY_SLOTS {
        if is_slot_open(frame, i) {
            return Some(i);
        }
    }
    None
}
