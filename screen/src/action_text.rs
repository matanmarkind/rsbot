use crate::fuzzy_pixels::{
    action_text_blue, action_text_green, action_text_orange, action_text_white, action_text_yellow,
};
use crate::types::*;
use crate::{pixels, Frame};
use std::fmt;
use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action.
///
/// These are all expected to be constants, so the lifetimes will be static.
///
/// TODO: Make letter private and word public.
pub struct Character {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Characters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: Vec<DeltaPosition>,

    /// Character represented.
    pub display: &'static str,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

pub struct Text {
    pub letters: Vec<(Character, FuzzyPixel)>,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.letters
                .iter()
                .map(|l| l.0.display)
                .collect::<Vec<&str>>()
                .join("")
        )
    }
}

/// Characters should all be biased in the same way. For example best to set all
/// letters to the default is to be on the left side of a column. If the letter
/// spans both the leftmost and rightmost pixel when there is perfect placement
/// it is harder to handle the shifting.

mod letters {
    use super::Character;
    use util::*;

    pub fn upper_a() -> Character {
        Character {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 3, dy: 2 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 5, dy: 11 },
                DeltaPosition { dx: 5, dy: 8 },
                DeltaPosition { dx: 5, dy: 5 },
            ],

            display: "A",
        }
    }

    pub fn upper_b() -> Character {
        Character {
            width: 8, // or 9...

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 4 },
                DeltaPosition { dx: 4, dy: 9 },
            ],

            display: "B",
        }
    }

    pub fn upper_c() -> Character {
        Character {
            width: 9, // 8 ?

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 1, dy: 3 },
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 3, dy: 2 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 4, dy: 3 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "C",
        }
    }

    pub fn upper_d() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 6 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "D",
        }
    }

    pub fn upper_f() -> Character {
        Character {
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

            display: "F",
        }
    }

    pub fn upper_k() -> Character {
        Character {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 3, dy: 4 },
                DeltaPosition { dx: 3, dy: 7 },
                DeltaPosition { dx: 5, dy: 3 },
                DeltaPosition { dx: 6, dy: 10 },
            ],

            display: "K",
        }
    }

    pub fn upper_m() -> Character {
        Character {
            width: 10,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 6, dy: 3 },
                DeltaPosition { dx: 6, dy: 7 },
                DeltaPosition { dx: 6, dy: 11 },
            ],

            display: "M",
        }
    }

    pub fn upper_n() -> Character {
        Character {
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

            display: "N",
        }
    }

    pub fn upper_o() -> Character {
        Character {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 1, dy: 3 },
                DeltaPosition { dx: 3, dy: 2 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 10 },
                DeltaPosition { dx: 5, dy: 3 },
                DeltaPosition { dx: 6, dy: 6 },
            ],

            display: "O",
        }
    }

    pub fn upper_r() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 7 },
                DeltaPosition { dx: 4, dy: 4 },
                DeltaPosition { dx: 5, dy: 10 },
            ],

            display: "R",
        }
    }

    pub fn upper_s() -> Character {
        Character {
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

            display: "S",
        }
    }

    pub fn upper_t() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box. T seems to be the
            // only letter that starts at screen_left+4 instead of 5. Currently I'm
            // not going to change everything to have the letters start at +4
            // instead of +5.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 5 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 2 },
            ],

            display: "T",
        }
    }

    pub fn upper_u() -> Character {
        Character {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 2 },
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 5, dy: 2 },
                DeltaPosition { dx: 5, dy: 5 },
                DeltaPosition { dx: 5, dy: 8 },
                DeltaPosition { dx: 5, dy: 11 },
            ],

            display: "U",
        }
    }

    pub fn upper_w() -> Character {
        Character {
            width: 10,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 1, dy: 10 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 3, dy: 7 },
                DeltaPosition { dx: 5, dy: 10 },
                DeltaPosition { dx: 6, dy: 3 },
                DeltaPosition { dx: 6, dy: 6 },
            ],

            display: "W",
        }
    }

    pub fn lower_a() -> Character {
        Character {
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

            display: "a",
        }
    }

    pub fn lower_b() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 2 },
                DeltaPosition { dx: 1, dy: 5 },
                DeltaPosition { dx: 1, dy: 8 },
                DeltaPosition { dx: 1, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 11 },
                DeltaPosition { dx: 5, dy: 9 },
            ],

            display: "b",
        }
    }

    pub fn lower_c() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 11 },
            ],

            display: "c",
        }
    }

    pub fn lower_d() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 9 },
                DeltaPosition { dx: 2, dy: 7 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 2 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "d",
        }
    }

    pub fn lower_e() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 9 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 7 },
            ],

            display: "e",
        }
    }

    pub fn lower_h() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "h",
        }
    }

    pub fn lower_i() -> Character {
        Character {
            width: 4,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 10 },
            ],

            display: "i",
        }
    }

    pub fn lower_g() -> Character {
        Character {
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

            display: "g",
        }
    }

    pub fn lower_k() -> Character {
        Character {
            width: 7,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 4, dy: 11 },
            ],

            display: "k",
        }
    }

    pub fn lower_l() -> Character {
        Character {
            width: 4,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 2 },
                DeltaPosition { dx: 1, dy: 5 },
                DeltaPosition { dx: 1, dy: 8 },
                DeltaPosition { dx: 1, dy: 11 },
            ],

            display: "l",
        }
    }

    pub fn lower_m() -> Character {
        Character {
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

            display: "m",
        }
    }

    pub fn lower_n() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 4, dy: 7 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "n",
        }
    }

    pub fn lower_o() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 8 },
            ],

            display: "o",
        }
    }

    pub fn lower_p() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 0, dy: 13 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 8 },
            ],

            display: "p",
        }
    }

    pub fn lower_r() -> Character {
        Character {
            width: 7,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 11 },
                DeltaPosition { dx: 3, dy: 6 },
            ],

            display: "r",
        }
    }

    pub fn lower_s() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 8 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 6 },
                DeltaPosition { dx: 4, dy: 9 },
            ],

            display: "s",
        }
    }

    pub fn lower_t() -> Character {
        Character {
            width: 6,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 3, dy: 6 },
                DeltaPosition { dx: 3, dy: 11 },
            ],

            display: "t",
        }
    }

    pub fn lower_u() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 6 },
                DeltaPosition { dx: 4, dy: 10 },
            ],

            display: "t",
        }
    }

    pub fn lower_v() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 1, dy: 8 },
                DeltaPosition { dx: 2, dy: 10 },
                DeltaPosition { dx: 3, dy: 8 },
                DeltaPosition { dx: 4, dy: 6 },
            ],

            display: "v",
        }
    }

    pub fn lower_w() -> Character {
        Character {
            width: 9,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 1, dy: 6 },
                DeltaPosition { dx: 1, dy: 9 },
                DeltaPosition { dx: 4, dy: 8 },
                DeltaPosition { dx: 6, dy: 6 },
                DeltaPosition { dx: 6, dy: 9 },
            ],

            display: "w",
        }
    }

    pub fn lower_z() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 6 },
                DeltaPosition { dx: 0, dy: 11 },
                DeltaPosition { dx: 2, dy: 6 },
                DeltaPosition { dx: 2, dy: 8 },
                DeltaPosition { dx: 2, dy: 11 },
                DeltaPosition { dx: 4, dy: 6 },
                DeltaPosition { dx: 4, dy: 11 },
            ],

            display: "z",
        }
    }

    pub fn space() -> Character {
        Character {
            width: 5,

            // Delta from the top left corner of the letters box.
            checkpoints: Vec::<DeltaPosition>::new(),

            display: " ",
        }
    }

    /// Sometimes the first letter is shifted, so put this at the start to allow
    /// offsetting the beginning.
    pub fn start() -> Character {
        Character {
            width: 0,

            // Delta from the top left corner of the letters box.
            checkpoints: Vec::<DeltaPosition>::new(),

            display: "",
        }
    }

    pub fn forward_slash() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 10 },
                DeltaPosition { dx: 1, dy: 7 },
                DeltaPosition { dx: 2, dy: 4 },
                DeltaPosition { dx: 3, dy: 1 },
            ],

            display: "/",
        }
    }

    pub fn hyphen() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 7 },
                DeltaPosition { dx: 1, dy: 7 },
                DeltaPosition { dx: 2, dy: 7 },
                DeltaPosition { dx: 3, dy: 7 },
                DeltaPosition { dx: 4, dy: 7 },
                DeltaPosition { dx: 5, dy: 7 },
            ],

            display: "-",
        }
    }

    pub fn greater_than() -> Character {
        Character {
            width: 8,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 3 },
                DeltaPosition { dx: 0, dy: 9 },
                DeltaPosition { dx: 2, dy: 4 },
                DeltaPosition { dx: 2, dy: 8 },
                DeltaPosition { dx: 5, dy: 6 },
            ],

            display: ">",
        }
    }

    pub fn open_paren() -> Character {
        Character {
            width: 6,

            // Delta from the top left corner of the letters box.
            checkpoints: vec![
                DeltaPosition { dx: 0, dy: 5 },
                DeltaPosition { dx: 0, dy: 8 },
                DeltaPosition { dx: 2, dy: 2 },
                DeltaPosition { dx: 2, dy: 11 },
            ],

            display: "(",
        }
    }
}
use letters::*;

pub fn smith_anvil() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_s(), action_text_white()),
            (lower_m(), action_text_white()),
            (lower_i(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_h(), action_text_white()),
            (space(), action_text_white()),
            (upper_a(), action_text_blue()),
            (lower_n(), action_text_blue()),
            (lower_v(), action_text_blue()),
            (lower_i(), action_text_blue()),
            (lower_l(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn attack_chicken() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_a(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_a(), action_text_white()),
            (lower_c(), action_text_white()),
            (lower_k(), action_text_white()),
            (space(), action_text_white()),
            (upper_c(), action_text_yellow()),
            (lower_h(), action_text_yellow()),
            (lower_i(), action_text_yellow()),
            (lower_c(), action_text_yellow()),
            (lower_k(), action_text_yellow()),
            (lower_e(), action_text_yellow()),
            (lower_n(), action_text_yellow()),
            // (space(), action_text_white()),
            // (space(), action_text_white()),
            // (open_paren(), action_text_green()),
        ],
    }
}

pub fn attack_cow() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_a(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_a(), action_text_white()),
            (lower_c(), action_text_white()),
            (lower_k(), action_text_white()),
            (space(), action_text_white()),
            (upper_c(), action_text_yellow()),
            (lower_o(), action_text_yellow()),
            (lower_w(), action_text_yellow()),
            // (space(), action_text_white()),
            // (space(), action_text_white()),
            // (open_paren(), action_text_green()),
        ],
    }
}

pub fn attack_al_kharid_warrior() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_a(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_t(), action_text_white()),
            (lower_a(), action_text_white()),
            (lower_c(), action_text_white()),
            (lower_k(), action_text_white()),
            (space(), action_text_white()),
            (upper_a(), action_text_yellow()),
            (lower_l(), action_text_yellow()),
            (hyphen(), action_text_yellow()),
            (upper_k(), action_text_yellow()),
            (lower_h(), action_text_yellow()),
            (lower_a(), action_text_yellow()),
            (lower_r(), action_text_yellow()),
            (lower_i(), action_text_yellow()),
            (lower_d(), action_text_yellow()),
            (space(), action_text_white()),
            (lower_w(), action_text_yellow()),
            (lower_a(), action_text_yellow()),
            (lower_r(), action_text_yellow()),
            (lower_r(), action_text_yellow()),
            (lower_i(), action_text_yellow()),
            (lower_o(), action_text_yellow()),
            (lower_r(), action_text_yellow()),
            // (space(), action_text_white()),
            // (space(), action_text_white()),
            // (open_paren(), action_text_green()),
        ],
    }
}

pub fn use_raw_shrimp_rightarrow_fire() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_u(), action_text_white()),
            (lower_s(), action_text_white()),
            (lower_e(), action_text_white()),
            (space(), action_text_white()),
            (upper_r(), action_text_orange()),
            (lower_a(), action_text_orange()),
            (lower_w(), action_text_orange()),
            (space(), action_text_white()),
            (lower_s(), action_text_orange()),
            (lower_h(), action_text_orange()),
            (lower_r(), action_text_orange()),
            (lower_i(), action_text_orange()),
            (lower_m(), action_text_orange()),
            (lower_p(), action_text_orange()),
            (lower_s(), action_text_orange()),
            (space(), action_text_white()),
            (hyphen(), action_text_white()),
            (greater_than(), action_text_white()),
            (space(), action_text_white()),
            (upper_f(), action_text_blue()),
            (lower_i(), action_text_blue()),
            (lower_r(), action_text_blue()),
            (lower_e(), action_text_blue()),
        ],
    }
}

pub fn use_raw_anchovies_rightarrow_fire() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_u(), action_text_white()),
            (lower_s(), action_text_white()),
            (lower_e(), action_text_white()),
            (space(), action_text_white()),
            (upper_r(), action_text_orange()),
            (lower_a(), action_text_orange()),
            (lower_w(), action_text_orange()),
            (space(), action_text_white()),
            (lower_a(), action_text_orange()),
            (lower_n(), action_text_orange()),
            (lower_c(), action_text_orange()),
            (lower_h(), action_text_orange()),
            (lower_o(), action_text_orange()),
            (lower_v(), action_text_orange()),
            (lower_i(), action_text_orange()),
            (lower_e(), action_text_orange()),
            (lower_s(), action_text_orange()),
            (space(), action_text_white()),
            (hyphen(), action_text_white()),
            (greater_than(), action_text_white()),
            (space(), action_text_white()),
            (upper_f(), action_text_blue()),
            (lower_i(), action_text_blue()),
            (lower_r(), action_text_blue()),
            (lower_e(), action_text_blue()),
        ],
    }
}

pub fn use_uncooked_pizza_rightarrow_range() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_u(), action_text_white()),
            (lower_s(), action_text_white()),
            (lower_e(), action_text_white()),
            (space(), action_text_white()),
            (upper_u(), action_text_orange()),
            (lower_n(), action_text_orange()),
            (lower_c(), action_text_orange()),
            (lower_o(), action_text_orange()),
            (lower_o(), action_text_orange()),
            (lower_k(), action_text_orange()),
            (lower_e(), action_text_orange()),
            (lower_d(), action_text_orange()),
            (space(), action_text_white()),
            (lower_p(), action_text_orange()),
            (lower_i(), action_text_orange()),
            (lower_z(), action_text_orange()),
            (lower_z(), action_text_orange()),
            (lower_a(), action_text_orange()),
            (space(), action_text_white()),
            (hyphen(), action_text_white()),
            (greater_than(), action_text_white()),
            (space(), action_text_white()),
            (upper_r(), action_text_blue()),
            (lower_a(), action_text_blue()),
            (lower_n(), action_text_blue()),
            (lower_g(), action_text_blue()),
            (lower_e(), action_text_blue()),
        ],
    }
}

pub fn smelt_furnace() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_s(), action_text_white()),
            (lower_m(), action_text_white()),
            (lower_e(), action_text_white()),
            (lower_l(), action_text_white()),
            (lower_t(), action_text_white()),
            (space(), action_text_white()),
            (upper_f(), action_text_blue()),
            (lower_u(), action_text_blue()),
            (lower_r(), action_text_blue()),
            (lower_n(), action_text_blue()),
            (lower_a(), action_text_blue()),
            (lower_c(), action_text_blue()),
            (lower_e(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn bank_bank_booth() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_b(), action_text_white()),
            (lower_a(), action_text_white()),
            (lower_n(), action_text_white()),
            (lower_k(), action_text_white()),
            (space(), action_text_white()),
            (upper_b(), action_text_blue()),
            (lower_a(), action_text_blue()),
            (lower_n(), action_text_blue()),
            (lower_k(), action_text_blue()),
            (space(), action_text_white()),
            (lower_b(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_t(), action_text_blue()),
            (lower_h(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn mine_rocks() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_m(), action_text_white()),
            (lower_i(), action_text_white()),
            (lower_n(), action_text_white()),
            (lower_e(), action_text_white()),
            (space(), action_text_white()),
            (upper_r(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_c(), action_text_blue()),
            (lower_k(), action_text_blue()),
            (lower_s(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn open_door() -> Text {
    Text {
        letters: vec![
            (upper_o(), action_text_white()),
            (lower_p(), action_text_white()),
            (lower_e(), action_text_white()),
            (lower_n(), action_text_white()),
            (space(), action_text_white()),
            (upper_d(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_r(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn chop_down_tree() -> Text {
    Text {
        letters: vec![
            (upper_c(), action_text_white()),
            (lower_h(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_p(), action_text_white()),
            (space(), action_text_white()),
            (lower_d(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_w(), action_text_white()),
            (lower_n(), action_text_white()),
            (space(), action_text_white()),
            (upper_t(), action_text_blue()),
            (lower_r(), action_text_blue()),
            (lower_e(), action_text_blue()),
            (lower_e(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn chop_down_oak() -> Text {
    Text {
        letters: vec![
            (upper_c(), action_text_white()),
            (lower_h(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_p(), action_text_white()),
            (space(), action_text_white()),
            (lower_d(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_w(), action_text_white()),
            (lower_n(), action_text_white()),
            (space(), action_text_white()),
            (upper_o(), action_text_blue()),
            (lower_a(), action_text_blue()),
            (lower_k(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn chop_down_willow() -> Text {
    Text {
        letters: vec![
            (upper_c(), action_text_white()),
            (lower_h(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_p(), action_text_white()),
            (space(), action_text_white()),
            (lower_d(), action_text_white()),
            (lower_o(), action_text_white()),
            (lower_w(), action_text_white()),
            (lower_n(), action_text_white()),
            (space(), action_text_white()),
            (upper_w(), action_text_blue()),
            (lower_i(), action_text_blue()),
            (lower_l(), action_text_blue()),
            (lower_l(), action_text_blue()),
            (lower_o(), action_text_blue()),
            (lower_w(), action_text_blue()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
        ],
    }
}

pub fn small_net_fishing_spot() -> Text {
    Text {
        letters: vec![
            (start(), action_text_white()),
            (upper_s(), action_text_white()),
            (lower_m(), action_text_white()),
            (lower_a(), action_text_white()),
            (lower_l(), action_text_white()),
            (lower_l(), action_text_white()),
            (space(), action_text_white()),
            (upper_n(), action_text_white()),
            (lower_e(), action_text_white()),
            (lower_t(), action_text_white()),
            (space(), action_text_white()),
            (upper_f(), action_text_yellow()),
            (lower_i(), action_text_yellow()),
            (lower_s(), action_text_yellow()),
            (lower_h(), action_text_yellow()),
            (lower_i(), action_text_yellow()),
            (lower_n(), action_text_yellow()),
            (lower_g(), action_text_yellow()),
            (space(), action_text_white()),
            (lower_s(), action_text_yellow()),
            (lower_p(), action_text_yellow()),
            (lower_o(), action_text_yellow()),
            (lower_t(), action_text_yellow()),
            (space(), action_text_white()),
            (forward_slash(), action_text_white()),
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
    action_text: &Text,
    action_text_top_left: Position,
) -> bool {
    let num_letter_mistmatches = check_action_letters_impl(
        frame,
        &action_text.letters[..],
        action_text_top_left,
        &[0, -1, 1, -2, 2],
    );
    action_text.letters.len() > 10 * num_letter_mistmatches
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
    letter_and_pixels: &[(Character, FuzzyPixel)],
    mut letter_offset: Position,
    space_offsets: &[i32],
) -> usize {
    let mut num_letter_mistmatches = 0;

    for (i, (letter, expected_pixel)) in letter_and_pixels.iter().enumerate() {
        // println!("check_letters_for_space -- next letter");
        for DeltaPosition { dx, dy } in letter.checkpoints.iter() {
            let pos = Position {
                x: letter_offset.x + dx,
                y: letter_offset.y + dy,
            };
            if !frame.check_loose_pixel(&pos, &expected_pixel) {
                // dbg!(&pos, &expected_pixel);
                // println!("check_letters_for_space -- no match");
                num_letter_mistmatches += 1;
                break;
            }
        }
        letter_offset = Position {
            x: letter_offset.x + letter.width,
            y: letter_offset.y,
        };

        if letter.checkpoints.is_empty() {
            // dbg!(&num_letter_mistmatches);
            let min_mismatches = space_offsets
                .iter()
                .map(|space_offset| {
                    check_action_letters_impl(
                        frame,
                        &letter_and_pixels[i + 1..],
                        Position {
                            x: letter_offset.x + space_offset,
                            y: letter_offset.y,
                        },
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
    frame: &impl Frame,
    fpath: &str,
    letter_and_pixels: &[(Character, FuzzyPixel)],
    action_text_top_left: Position,
) -> std::thread::JoinHandle<()> {
    let mut img = frame.to_owned();

    let mut x_offset = action_text_top_left.x;
    for (letter, _) in letter_and_pixels {
        for util::DeltaPosition { dx, dy } in letter.checkpoints.iter() {
            let pos = util::Position {
                x: x_offset + dx,
                y: action_text_top_left.y + dy,
            };
            img.recolor_pixel(&pos, &pixels::red());
        }
        x_offset += letter.width;
    }

    // Spawn image saving to another thread since it takes a very long time.
    let fpath = fpath.to_string();
    std::thread::spawn(move || {
        img.flip_to_rgb();
        img.save(fpath.as_str());
    })
}
