use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

// TODO: remove Add/Sub that are based on references. These types are Copy so we
// don't need to worry about that.

/// Types that are used by multiple crates. For example mouse and screen
/// shouldn't import from each other to we don't put Position in either of them.

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
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

impl Sub for Position {
    type Output = DeltaPosition;

    fn sub(self, other: Position) -> DeltaPosition {
        DeltaPosition {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&DeltaPosition> for &Position {
    type Output = Position;

    fn add(self, other: &DeltaPosition) -> Position {
        Position {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl Add<DeltaPosition> for Position {
    type Output = Position;

    fn add(self, other: DeltaPosition) -> Position {
        Position {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl Sub<DeltaPosition> for Position {
    type Output = Position;

    fn sub(self, other: DeltaPosition) -> Position {
        Position {
            x: self.x - other.dx,
            y: self.y - other.dy,
        }
    }
}

impl FromStr for Position {
    type Err = ParseIntError;

    /// Input is expected to be "x,y" without anything around (e.g. no "(x,y)")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(",").collect();
        Ok(Position {
            x: coords[0].parse::<i32>()?,
            y: coords[1].parse::<i32>()?,
        })
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
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

impl Add for DeltaPosition {
    type Output = DeltaPosition;

    fn add(self, other: DeltaPosition) -> DeltaPosition {
        DeltaPosition {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

impl Sub for DeltaPosition {
    type Output = DeltaPosition;

    fn sub(self, other: DeltaPosition) -> DeltaPosition {
        DeltaPosition {
            dx: self.dx - other.dx,
            dy: self.dy - other.dy,
        }
    }
}

impl Mul<f32> for DeltaPosition {
    type Output = DeltaPosition;

    fn mul(self, factor: f32) -> DeltaPosition {
        DeltaPosition {
            dx: (self.dx as f32 * factor).round() as i32,
            dy: (self.dy as f32 * factor).round() as i32,
        }
    }
}

impl FromStr for DeltaPosition {
    type Err = ParseIntError;

    /// Input is expected to be "x,y" without anything around (e.g. no "(x,y)")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<&str> = s.trim().split(",").collect();
        Ok(DeltaPosition {
            dx: dims[0].parse::<i32>()?,
            dy: dims[1].parse::<i32>()?,
        })
    }
}

/// Bounding box made of top_left (included), past_bottom_right (excluded).
pub struct BoundingBox(pub Position, pub Position);
