use crate::locations::*;
use crate::types::*;
use crate::Frame;
use util::*;

pub const ACTION_WHITE: FuzzyPixel = FuzzyPixel {
    blue_min: 180,
    blue_max: 255,
    green_min: 180,
    green_max: 255,
    red_min: 180,
    red_max: 255,
};

pub const ACTION_BLUE: FuzzyPixel = FuzzyPixel {
    blue_min: 180,
    blue_max: 255,
    green_min: 180,
    green_max: 255,
    red_min: 0,
    red_max: 25,
};

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action.
///
/// These are all expected to be constants, so the lifetimes will be static.
pub struct ActionLetter<'a> {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Letters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: &'a [DeltaPosition],
}

pub const UPPER_C: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 0, dy: 4 },
        DeltaPosition { dx: 3, dy: 0 },
        DeltaPosition { dx: 3, dy: 9 },
        DeltaPosition { dx: 4, dy: 1 },
        DeltaPosition { dx: 4, dy: 8 },
    ],
};

pub const UPPER_T: ActionLetter = ActionLetter {
    width: 7,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 0, dy: 0 },
        DeltaPosition { dx: 2, dy: 0 },
        DeltaPosition { dx: 2, dy: 3 },
        DeltaPosition { dx: 2, dy: 9 },
        DeltaPosition { dx: 4, dy: 0 },
    ],
};

pub const LOWER_D: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 7 },
        DeltaPosition { dx: 3, dy: 5 },
        DeltaPosition { dx: 3, dy: 9 },
        DeltaPosition { dx: 5, dy: 0 },
        DeltaPosition { dx: 5, dy: 8 },
    ],
};

pub const LOWER_E: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 4 },
        DeltaPosition { dx: 1, dy: 9 },
        DeltaPosition { dx: 3, dy: 4 },
        DeltaPosition { dx: 3, dy: 7 },
        DeltaPosition { dx: 3, dy: 9 },
        DeltaPosition { dx: 4, dy: 5 },
    ],
};

pub const LOWER_H: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 1 },
        DeltaPosition { dx: 1, dy: 8 },
        DeltaPosition { dx: 3, dy: 4 },
        DeltaPosition { dx: 5, dy: 8 },
    ],
};

pub const LOWER_N: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 4 },
        DeltaPosition { dx: 1, dy: 7 },
        DeltaPosition { dx: 3, dy: 4 },
        DeltaPosition { dx: 5, dy: 4 },
        DeltaPosition { dx: 5, dy: 7 },
    ],
};

pub const LOWER_O: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 6 },
        DeltaPosition { dx: 3, dy: 4 },
        DeltaPosition { dx: 3, dy: 9 },
        DeltaPosition { dx: 5, dy: 6 },
    ],
};

pub const LOWER_P: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 6 },
        DeltaPosition { dx: 1, dy: 11 },
        DeltaPosition { dx: 3, dy: 4 },
        DeltaPosition { dx: 3, dy: 9 },
        DeltaPosition { dx: 5, dy: 6 },
    ],
};

pub const LOWER_R: ActionLetter = ActionLetter {
    width: 7,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 4 },
        DeltaPosition { dx: 1, dy: 9 },
        DeltaPosition { dx: 3, dy: 4 },
    ],
};

pub const LOWER_W: ActionLetter = ActionLetter {
    width: 9,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 4 },
        DeltaPosition { dx: 1, dy: 7 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 6, dy: 4 },
        DeltaPosition { dx: 6, dy: 7 },
    ],
};

pub const SPACE: ActionLetter = ActionLetter {
    width: 5,

    // Delta from the top left corner of the letters box.
    checkpoints: &[],
};

// Spaces between words seem to be of different sizes. Therefore we try matching
// with different spaces.
fn check_letters_for_space(
    frame: &impl Frame,
    letter_and_pixels: &[(ActionLetter, FuzzyPixel)],
    space_offset: i32,
) -> bool {
    let mut x_offset = TOP_LEFT_ACTION_TEXT.x;
    let mut num_letter_mistmatches = 0;

    for (letter, expected_pixel) in letter_and_pixels {
        for DeltaPosition { dx, dy } in letter.checkpoints {
            let pos = Position {
                x: x_offset + dx,
                y: TOP_LEFT_ACTION_TEXT.y + dy,
            };
            if !frame.check_loose_pixel(&pos, &expected_pixel) {
                // dbg!(&pos, &expected_pixel);
                num_letter_mistmatches += 1;
                break;
            }
        }
        dbg!(&num_letter_mistmatches);
        x_offset += letter.width;

        if letter.checkpoints.is_empty() {
            // If 'letter' was a space, then offset it.
            println!("space");
            x_offset += space_offset;
        }
    }

    // As the number of characters increases, the likelihood of error
    // increases. Unfortunately this may not help with the main risk,
    // which is close substitutes (is Chop down Tree vs. Chop down Oak)
    letter_and_pixels.len() > 10 * num_letter_mistmatches
}

/// Check if the action described in the top left matches what we want to be
/// doing.
///
/// When the mouse hovers over an object that can be acted upon, the top
/// left corner of the screen displays in words describes the action a left
/// click will cause us to take.
pub fn check_action_letters(
    frame: &impl Frame,
    letter_and_pixels: &[(ActionLetter, FuzzyPixel)],
) -> bool {
    for space_offset in &[0, -1, 1] {
        println!("space_offset={}", space_offset);
        if check_letters_for_space(frame, letter_and_pixels, *space_offset) {
            return true;
        }
    }
    false
}
