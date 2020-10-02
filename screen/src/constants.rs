use crate::ActionLetter;
use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action. The letter boxes will begin at this point.
pub const TOP_LEFT_ACTION_TEXT: Position = Position { x: 968, y: 57 };

pub const LETTER_WHITE: FuzzyPixel = FuzzyPixel {
    blue_min: 221,
    blue_max: 221,
    green_min: 221,
    green_max: 221,
    red_min: 221,
    red_max: 221,
};

pub const BLACK: FuzzyPixel = FuzzyPixel {
    blue_min: 0,
    blue_max: 0,
    green_min: 0,
    green_max: 0,
    red_min: 0,
    red_max: 0,
};

/// Letters - name the variable the name of the letter, case sensitive.
///
pub const C: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 6 },
        DeltaPosition { dx: 4, dy: 1 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 3 },
        DeltaPosition { dx: 6, dy: 10 },
    ],
};
