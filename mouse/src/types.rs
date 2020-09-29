use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::ops::Sub;
use util::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Location {
    pub time_us: i64,
    pub x: i32,
    pub y: i32,
}

// Implementing subtraction by reference to avoid: a. consume values on
// subtraction, which is surprising and annoying. b. Automatically copying which
// is also surprising to user and seems inefficient. The downside is that this
// creates a weird usage syntax (&a - &b).
impl Sub for &Location {
    type Output = Location;

    fn sub(self, other: &Location) -> Location {
        Location {
            time_us: self.time_us - other.time_us,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// TODO: consider adding max_displacement_x, max_displacement_y. This is the
// maximum change in (x, y) that will happen when following the path. Used to
// avoid hitting the edge of the screen. Not clear if this would be helpful
// though with rotation.

// Ordering and equality is done by the distance only.
#[derive(PartialOrd, Debug, Serialize, Deserialize)]
pub struct PathSummary {
    pub distance: i32,
    // Angle of the line from x axis in radians [0, 2PI)
    pub angle_rads: f32,
}

impl PartialEq for PathSummary {
    fn eq(&self, other: &PathSummary) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PathSummary {}

impl Ord for PathSummary {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub type MousePath = Vec<DeltaPosition>;
pub type MousePaths = BTreeMap<PathSummary, MousePath>;
