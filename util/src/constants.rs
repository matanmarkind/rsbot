#![allow(warnings)] // Don't warn for unused variables... At least for now.

use crate::types::*;
use std::time::Duration;

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

/// Amount of time to wait if waiting for the screen to change in response to an
/// action (e.g. if we closed the chatbox how long ot wait to check it has been
/// closed).
///
/// I think I remember seeing somewhere this should be 6x per second. To be safe
/// I am going with 1/3 of a second since the downside of missing a redraw is
/// worse than the delay of an extra hundred ms.
pub const REDRAW_TIME: Duration = Duration::from_millis(333);
