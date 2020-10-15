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

/// If InputBot is told to move somewhere, we want this to always succeed.
/// Sometimes the MouseMover needs multiple runs (for instance we don't handle
/// hitting the edge of the screen well which can cause a failure.) To avoid an
/// infinite loop though we define a timeout here.
pub const MOVE_TO_TIMEOUT: Duration = Duration::from_secs(60);
