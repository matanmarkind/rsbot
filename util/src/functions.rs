use crate::types::*;
use rand::{thread_rng, Rng};

/// 'top_left' - top left corner of the image (included). (x,y) represent the
/// top/leftmost row/column of the frame to search in.
///
/// 'past_bottom_right' - bottom right of the image (excluded). (x,y) represent
/// one past the bottom/rightmost row/column of the frame to search in.
///
/// Returns the position of the first pixel found which matches the criteria. If
/// no pixel is found return None.
pub fn random_position(top_left: &Position, dimensions: &DeltaPosition) -> Position {
    let mut rng = thread_rng();
    Position {
        x: rng.gen_range(top_left.x, top_left.x + dimensions.dx),
        y: rng.gen_range(top_left.y, top_left.y + dimensions.dy),
    }
}

pub fn random_position_polar(middle: Position, radius: f32) -> Position {
    let mut rng = thread_rng();
    let r = rng.gen_range(0.0, radius);
    let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
    polar_to_cartesian(middle, r, angle)
}

pub fn polar_to_cartesian(middle: Position, radius: f32, angle_rad: f32) -> Position {
    Position {
        x: (middle.x as f32 + radius * angle_rad.cos()).round() as i32,
        y: (middle.y as f32 + radius * angle_rad.sin()).round() as i32,
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
