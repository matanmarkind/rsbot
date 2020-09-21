use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Location {
    pub time_us: i64,
    pub x: i32,
    pub y: i32,
}

pub const ZERO_LOC: Location = Location {
    time_us: 0,
    x: 0,
    y: 0,
};

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

// Ordering and equality is done by the distance only.
#[derive(PartialOrd, Debug, Serialize)]
pub struct PathSummary {
    pub distance: i32,
    pub avg_time_us: i32,
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

#[derive(PartialEq, PartialOrd, Debug, Serialize)]
pub struct DeltaPosition {
    pub dx: i32,
    pub dy: i32,
}

impl DeltaPosition {
    pub fn new() -> DeltaPosition {
        DeltaPosition { dx: 0, dy: 0 }
    }
}

impl Add for &DeltaPosition {
    type Output = DeltaPosition;

    fn add(self, other: &DeltaPosition) -> DeltaPosition {
        DeltaPosition {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

pub type MousePath = Vec<DeltaPosition>;
pub type MousePaths = BTreeMap<PathSummary, MousePath>;
