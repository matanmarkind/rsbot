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
pub struct ActionLetter {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Letters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: Vec<DeltaPosition>,
}

pub mod letters {
    use super::ActionLetter;
    use util::*;

    pub fn upper_c() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 3, dy: 2 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 4, dy: 3 },
                DeltaPosition { dx: 4, dy: 10 },
            ],
        }
    }

    pub fn upper_o() -> ActionLetter {
        ActionLetter {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 2, dy: 10 },
                DeltaPosition { dx: 2, dy: 3 },
                DeltaPosition { dx: 4, dy: 2 },
                DeltaPosition { dx: 4, dy: 11 },
                DeltaPosition { dx: 6, dy: 10 },
                DeltaPosition { dx: 6, dy: 3 },
                DeltaPosition { dx: 7, dy: 6 },
            ],
        }
    }

    pub fn upper_t() -> ActionLetter {
        ActionLetter {
            width: 7,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 5 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 2 },
            ],
        }
    }

    pub fn lower_a() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 8 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 7 },
                DeltaPosition { dx: 5, dy: 10 },
            ],
        }
    }

    pub fn lower_d() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 9 },
                DeltaPosition { dx: 3, dy: 7 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 2 },
                DeltaPosition { dx: 5, dy: 10 },
            ],
        }
    }

    pub fn lower_e() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 9 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 4, dy: 7 },
            ],
        }
    }

    pub fn lower_h() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 3 },
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 5, dy: 10 },
            ],
        }
    }

    pub fn lower_k() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 3 },
                DeltaPosition { dx: 1, dy: 7 },
                DeltaPosition { dx: 1, dy: 11 },
                DeltaPosition { dx: 4, dy: 6 },
                DeltaPosition { dx: 5, dy: 11 },
            ],
        }
    }

    pub fn lower_n() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 9 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 5, dy: 6 },
                DeltaPosition { dx: 5, dy: 9 },
            ],
        }
    }

    pub fn lower_o() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 8 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 8 },
            ],
        }
    }

    pub fn lower_p() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 8 },
                DeltaPosition { dx: 1, dy: 13 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 8 },
            ],
        }
    }

    pub fn lower_r() -> ActionLetter {
        ActionLetter {
            width: 7,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
            ],
        }
    }

    pub fn lower_w() -> ActionLetter {
        ActionLetter {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 9 },
                DeltaPosition { dx: 4, dy: 8 },
                DeltaPosition { dx: 6, dy: 6 },
                DeltaPosition { dx: 6, dy: 9 },
            ],
        }
    }

    pub fn space() -> ActionLetter {
        ActionLetter {
            width: 5,

            // Delta from the top left corner of the letters box.
            checkpoints: Vec::<DeltaPosition>::new(),
        }
    }

    pub fn forward_slash() -> ActionLetter {
        ActionLetter {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 2, dy: 7 },
                DeltaPosition { dx: 3, dy: 4 },
                DeltaPosition { dx: 4, dy: 1 },
            ],
        }
    }
}

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
        for DeltaPosition { dx, dy } in letter.checkpoints.iter() {
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
        // dbg!(&num_letter_mistmatches);
        x_offset += letter.width;

        if letter.checkpoints.is_empty() {
            // If 'letter' was a space, then offset it.
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
        // println!("check_action_letters -- space_offset={}", space_offset);
        if check_letters_for_space(frame, letter_and_pixels, *space_offset) {
            return true;
        }
    }
    false
}
