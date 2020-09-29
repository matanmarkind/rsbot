use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

pub fn parse_position(src: &str) -> Result<Position, csv::Error> {
    // TODO: Find an easier way to deserialize...
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(src.as_bytes());
    let mut ret = Ok(Position { x: 0, y: 0 });
    for result in reader.deserialize::<Position>() {
        ret = result;
        break;
    }
    ret
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
