use crate::types::*;
use std::time::Duration;

/// Amount of time between mouse movements.
///
/// The time should be small enough that the mouse movements look natural, no
/// teleporting mouse. Going too small would make the timing between recordings
/// less stable, and would greatly increase the size of the paths.
///
/// While the values can be changed, they must remain constant for all phases:
/// recording, parsing, and usage.
pub const MIN_TIME_BETWEEN_LOCATIONS: Duration = Duration::from_millis(9);
pub const MAX_TIME_BETWEEN_LOCATIONS: Duration = Duration::from_millis(12);

pub const ZERO_LOC: Location = Location {
    time_us: 0,
    x: 0,
    y: 0,
};

// Limits on how far the mouse can move in a single jump. These should make
// sense based on how long we have between locations.

// Maximum distance we can teleport the mouse when 'cheating' towards to a .
// This should be on the order of a smallish DeltaPosition. Used by the
// controller.
pub const MAX_CHEAT_DISTANCE: i32 = 20;

/// Time to wait between press and release of mouse buttons.
pub const MIN_CLICK_WAIT: Duration = Duration::from_millis(100);
pub const MAX_CLICK_WAIT: Duration = Duration::from_millis(150);

/// Time to wait between press and release of keyboard buttons.
pub const MIN_PRESS_WAIT: Duration = Duration::from_millis(120);
pub const MAX_PRESS_WAIT: Duration = Duration::from_millis(170);
