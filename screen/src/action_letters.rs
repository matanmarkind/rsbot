use crate::types::*;
use crate::Frame;
use crate::{colors, locations};
use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action.
///
/// These are all expected to be constants, so the lifetimes will be static.
pub struct Letter {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Letters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: Vec<DeltaPosition>,
}

pub fn upper_c() -> Letter {
    Letter {
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

pub fn upper_f() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 2 },
            DeltaPosition { dx: 0, dy: 6 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 3, dy: 2 },
            DeltaPosition { dx: 3, dy: 6 },
            DeltaPosition { dx: 5, dy: 2 },
        ],
    }
}

pub fn upper_n() -> Letter {
    Letter {
        width: 9,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 2 },
            DeltaPosition { dx: 0, dy: 6 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 2, dy: 4 },
            DeltaPosition { dx: 4, dy: 8 },
            DeltaPosition { dx: 6, dy: 2 },
            DeltaPosition { dx: 6, dy: 6 },
            DeltaPosition { dx: 6, dy: 10 },
        ],
    }
}

pub fn upper_o() -> Letter {
    Letter {
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

pub fn upper_s() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 4 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 3, dy: 2 },
            DeltaPosition { dx: 3, dy: 6 },
            DeltaPosition { dx: 3, dy: 11 },
            DeltaPosition { dx: 5, dy: 3 },
            DeltaPosition { dx: 5, dy: 8 },
        ],
    }
}

pub fn upper_t() -> Letter {
    Letter {
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

pub fn lower_a() -> Letter {
    Letter {
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

pub fn lower_d() -> Letter {
    Letter {
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

pub fn lower_e() -> Letter {
    Letter {
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

pub fn lower_h() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 3 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 4, dy: 10 },
        ],
    }
}

pub fn lower_i() -> Letter {
    Letter {
        width: 4,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 5 },
            DeltaPosition { dx: 0, dy: 7 },
            DeltaPosition { dx: 0, dy: 10 },
        ],
    }
}

pub fn lower_g() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 8 },
            DeltaPosition { dx: 0, dy: 13 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 2, dy: 11 },
            DeltaPosition { dx: 2, dy: 14 },
            DeltaPosition { dx: 4, dy: 8 },
            DeltaPosition { dx: 4, dy: 13 },
        ],
    }
}

pub fn lower_k() -> Letter {
    Letter {
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

pub fn lower_l() -> Letter {
    Letter {
        width: 4,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 1, dy: 2 },
            DeltaPosition { dx: 1, dy: 5 },
            DeltaPosition { dx: 1, dy: 8 },
            DeltaPosition { dx: 1, dy: 11 },
        ],
    }
}

pub fn lower_m() -> Letter {
    Letter {
        width: 10,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 7 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 3, dy: 10 },
            DeltaPosition { dx: 5, dy: 6 },
            DeltaPosition { dx: 6, dy: 7 },
            DeltaPosition { dx: 6, dy: 10 },
        ],
    }
}

pub fn lower_n() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 6 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 4, dy: 7 },
            DeltaPosition { dx: 4, dy: 10 },
        ],
    }
}

pub fn lower_o() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 8 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 2, dy: 11 },
            DeltaPosition { dx: 4, dy: 8 },
        ],
    }
}

pub fn lower_p() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 8 },
            DeltaPosition { dx: 0, dy: 13 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 2, dy: 11 },
            DeltaPosition { dx: 4, dy: 8 },
        ],
    }
}

pub fn lower_r() -> Letter {
    Letter {
        width: 7,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 1, dy: 6 },
            DeltaPosition { dx: 1, dy: 11 },
            DeltaPosition { dx: 3, dy: 6 },
        ],
    }
}

pub fn lower_s() -> Letter {
    Letter {
        width: 9,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 7 },
            DeltaPosition { dx: 0, dy: 11 },
            DeltaPosition { dx: 2, dy: 6 },
            DeltaPosition { dx: 2, dy: 8 },
            DeltaPosition { dx: 2, dy: 11 },
            DeltaPosition { dx: 5, dy: 6 },
            DeltaPosition { dx: 5, dy: 9 },
        ],
    }
}

pub fn lower_t() -> Letter {
    Letter {
        width: 6,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 3 },
            DeltaPosition { dx: 0, dy: 6 },
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 3, dy: 6 },
            DeltaPosition { dx: 3, dy: 11 },
        ],
    }
}

pub fn lower_w() -> Letter {
    Letter {
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

pub fn space() -> Letter {
    Letter {
        width: 5,

        // Delta from the top left corner of the letters box.
        checkpoints: Vec::<DeltaPosition>::new(),
    }
}

/// Sometimes the first letter is shifted, so put this at the start to allow
/// offsetting the beginning.
pub fn start() -> Letter {
    Letter {
        width: 0,

        // Delta from the top left corner of the letters box.
        checkpoints: Vec::<DeltaPosition>::new(),
    }
}

pub fn forward_slash() -> Letter {
    Letter {
        width: 8,

        // Delta from the top left corner of the letters box.
        checkpoints: vec![
            DeltaPosition { dx: 0, dy: 10 },
            DeltaPosition { dx: 1, dy: 7 },
            DeltaPosition { dx: 2, dy: 4 },
            DeltaPosition { dx: 3, dy: 1 },
        ],
    }
}

/// Check if the action described in the top left matches what we want to be
/// doing.
///
/// When the mouse hovers over an object that can be acted upon, the top
/// left corner of the screen displays in words describes the action a left
/// click will cause us to take.
pub fn check_action_letters(
    frame: &impl Frame,
    letter_and_pixels: &[(Letter, FuzzyPixel)],
) -> bool {
    let num_letter_mistmatches = check_action_letters_impl(
        frame,
        letter_and_pixels,
        locations::TOP_LEFT_ACTION_TEXT.x,
        &[0, -1, 1],
    );
    letter_and_pixels.len() > 10 * num_letter_mistmatches
}

/// Search all the charachters until the next space (action letter with no
/// checkpoints). Then recursively call to this function from that point varying
/// the x_offset passed (aka the size of the space between words).
///
/// Variability in each space is [-1, 0, 1]. So the function complexity is
/// O(3**num_spaces).
///
/// Returns the number of letters that could not be matched.
pub fn check_action_letters_impl(
    frame: &impl Frame,
    letter_and_pixels: &[(Letter, FuzzyPixel)],
    mut x_offset: i32,
    space_offsets: &[i32],
) -> usize {
    let mut num_letter_mistmatches = 0;

    for (i, (letter, expected_pixel)) in letter_and_pixels.iter().enumerate() {
        // println!("check_letters_for_space -- next letter");
        for DeltaPosition { dx, dy } in letter.checkpoints.iter() {
            let pos = Position {
                x: x_offset + dx,
                y: locations::TOP_LEFT_ACTION_TEXT.y + dy,
            };
            if !frame.check_loose_pixel(&pos, &expected_pixel) {
                // dbg!(&pos, &expected_pixel);
                // println!("check_letters_for_space -- no match");
                num_letter_mistmatches += 1;
                break;
            }
        }
        x_offset += letter.width;

        if letter.checkpoints.is_empty() {
            // dbg!(&num_letter_mistmatches);
            let min_mismatches = space_offsets
                .iter()
                .map(|space_offset| {
                    check_action_letters_impl(
                        frame,
                        &letter_and_pixels[i + 1..],
                        x_offset + space_offset,
                        space_offsets,
                    )
                })
                .min()
                .unwrap_or(0);
            // dbg!(min_mismatches);
            num_letter_mistmatches += min_mismatches;
            break;
        }
    }
    num_letter_mistmatches
}

/// Used for debugging to show what check_action_letters does. Marks with a red
/// spot each pixel that represents a checkpoint for checking action words.
///
/// Shows space offset of 0 only, and colors the exact pixel, where the check is
/// done for a lose pixel.
pub fn mark_letters_and_save(
    frame: &impl crate::Frame,
    fpath: &str,
    letter_and_pixels: &[(Letter, FuzzyPixel)],
) -> std::thread::JoinHandle<()> {
    let mut img = frame.to_owned();

    let mut x_offset = locations::TOP_LEFT_ACTION_TEXT.x;
    for (letter, _) in letter_and_pixels {
        for util::DeltaPosition { dx, dy } in letter.checkpoints.iter() {
            let pos = util::Position {
                x: x_offset + dx,
                y: locations::TOP_LEFT_ACTION_TEXT.y + dy,
            };
            img.recolor_pixel(&pos, &colors::PURE_RED);
        }
        x_offset += letter.width;
    }

    // Spawn image saving to another thread since it takes a very long time.
    let fpath = fpath.to_string();
    std::thread::spawn(move || {
        img.crop(locations::WINDOW_TOP_LEFT, locations::WINDOW_DIMENSIONS)
            .flip_to_rgb();
        img.save(fpath.as_str());
    })
}
