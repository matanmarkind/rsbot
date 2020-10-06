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
