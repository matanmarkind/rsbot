use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Sub for &Position {
    type Output = DeltaPosition;

    fn sub(self, other: &Position) -> DeltaPosition {
        DeltaPosition {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct DeltaPosition {
    pub dx: i32,
    pub dy: i32,
}

impl DeltaPosition {
    pub fn new() -> DeltaPosition {
        DeltaPosition { dx: 0, dy: 0 }
    }

    // Total distance covered by the delta. Length of the vector from (0, 0) to (dx, dy).
    pub fn distance(&self) -> i32 {
        ((self.dx.pow(2) + self.dy.pow(2)) as f32).sqrt().round() as i32
    }

    // Calculate the angle from the positive x axis to a line pointed from (0, 0) to
    // (dx, dy). Results are on the range [0, 2PI).
    pub fn angle_rads(&self) -> f32 {
        // Correctly handles if only 1 is 0.
        if self.dx == 0 && self.dy == 0 {
            return 0.0;
        }

        let dx = self.dx as f32;
        let dy = self.dy as f32;
        let raw_angle = (dy / dx).atan();

        if dx >= 0.0 && dy >= 0.0 {
            // Q1
            raw_angle
        } else if dx <= 0.0 && dy >= 0.0 {
            // Q2
            std::f32::consts::PI + raw_angle
        } else if dx <= 0.0 && dy <= 0.0 {
            // Q3
            std::f32::consts::PI + raw_angle
        } else {
            // Q4
            2.0 * std::f32::consts::PI + raw_angle
        }
    }

    // Rotate the delta by 'angle_rads' radians from the x axis.
    pub fn rotate(&self, angle_rads: f32) -> DeltaPosition {
        let sin = angle_rads.sin();
        let cos = angle_rads.cos();
        let dx = self.dx as f32;
        let dy = self.dy as f32;

        DeltaPosition {
            dx: (dx * cos - dy * sin).round() as i32,
            dy: (dx * sin + dy * cos).round() as i32,
        }
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
