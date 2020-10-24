#![allow(warnings)] // Don't warn for unused variables... At least for now.

use crate::types::*;
use std::time::Duration;

/// Amount of time to wait if waiting for the screen to change in response to an
/// action (e.g. if we closed the chatbox how long ot wait to check it has been
/// closed).
///
/// I think I remember seeing somewhere this should be 6x per second. To be safe
/// I am going with 1/3 of a second since the downside of missing a redraw is
/// worse than the delay of an extra hundred ms.
pub const REDRAW_TIME: Duration = Duration::from_millis(333);
