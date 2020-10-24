use crate::{degrees_to_radians, polar_to_cartesian, radius_and_arclen_to_radians};
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

/// This struct defines an iterator which will move over a series of positions,
/// spiraling out from 'middle'. While it is a circular spiral, we can define a
/// radial range, so we only search a donut, and also an angular range so we
/// only search an arc.
#[derive(Debug, PartialEq)]
pub struct PositionIteratorCircularSpiral {
    // Constant State.

    // Center of the circle around which we will spiral out.
    middle: Position,
    // 1 + Max radius we will go to.
    end_radius: i32,
    // Minimum angle each circumference will start from.
    min_angle_rads: f32,
    // 1 + Maximum angle each circumference will travel to.
    end_angle_rads: f32,
    // Spacing between positions on this circumference. This is used so that as
    // we move away from middle the density of checks remains constant.
    spacing: i32,

    // Mutable State.

    // Current radius
    radius: i32,
    // Angle we are currently at.
    angle_rads: f32,
}

impl PositionIteratorCircularSpiral {
    pub fn new(
        middle: Position,
        min_radius: i32,
        d_radius: i32,
        min_angle_degrees: f32,
        d_angle_degrees: f32,
        spacing: i32,
    ) -> PositionIteratorCircularSpiral {
        assert!(min_radius > 0);
        assert!(d_radius >= 0);
        assert!(min_angle_degrees >= 0.0 && min_angle_degrees <= 360.0);
        assert!(d_angle_degrees >= 0.0 && d_angle_degrees + min_angle_degrees <= 360.0);
        assert!(spacing > 0);
        assert!((middle.x - (min_radius + d_radius - 1)) >= 0);
        assert!((middle.y - (min_radius + d_radius - 1)) >= 0);

        PositionIteratorCircularSpiral {
            // Constants.
            middle,
            end_radius: min_radius + d_radius,
            min_angle_rads: degrees_to_radians(min_angle_degrees),
            end_angle_rads: degrees_to_radians(min_angle_degrees + d_angle_degrees),
            spacing,

            // Mutable.
            radius: min_radius,
            angle_rads: degrees_to_radians(min_angle_degrees),
        }
    }
}

impl Iterator for PositionIteratorCircularSpiral {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.radius >= self.end_radius {
            return None;
        }

        let pos = polar_to_cartesian(self.middle, self.radius, self.angle_rads);

        let angle_delta_rads = radius_and_arclen_to_radians(self.radius, self.spacing);
        self.angle_rads += angle_delta_rads;
        if self.angle_rads >= self.end_angle_rads {
            // We have completed this arc, expand out to the next one.
            self.angle_rads = self.min_angle_rads;
            self.radius += self.spacing;
        }

        Some(pos)
    }
}
