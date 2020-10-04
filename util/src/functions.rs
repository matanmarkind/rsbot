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
pub fn random_position(top_left: &Position, past_bottom_right: &Position) -> Position {
    let mut rng = thread_rng();
    Position {
        x: rng.gen_range(top_left.x, past_bottom_right.x),
        y: rng.gen_range(top_left.y, past_bottom_right.y),
    }
}
