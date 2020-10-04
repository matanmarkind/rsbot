use crate::types::*;
use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action. The letter boxes will begin at this point.
pub const TOP_LEFT_ACTION_TEXT: Position = Position { x: 968, y: 57 };

// Each pixel is represented by 4 u8's, BGRA/RGBA. Each frame is a list of u8's.
pub const RAW_PIXEL_SIZE: usize = 4;

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

/// Letters - name the variable the name of the letter, case sensitive. They
/// don't need to be perfect (100% true positive & 0% false positive) since they
/// will be used together, so if the probabilities are good enough then the net
/// effect should be sufficient.
///
pub const UPPER_C: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 6 },
        DeltaPosition { dx: 4, dy: 2 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 3 },
        DeltaPosition { dx: 6, dy: 10 },
    ],
};

pub const UPPER_T: ActionLetter = ActionLetter {
    width: 7,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 1, dy: 2 },
        DeltaPosition { dx: 3, dy: 2 },
        DeltaPosition { dx: 3, dy: 5 },
        DeltaPosition { dx: 3, dy: 11 },
        DeltaPosition { dx: 5, dy: 2 },
    ],
};

pub const LOWER_D: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 9 },
        DeltaPosition { dx: 4, dy: 7 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 2 },
        DeltaPosition { dx: 6, dy: 10 },
    ],
};

pub const LOWER_E: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 6 },
        DeltaPosition { dx: 2, dy: 11 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 4, dy: 9 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 7 },
    ],
};

pub const LOWER_H: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 3 },
        DeltaPosition { dx: 2, dy: 10 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 6, dy: 10 },
    ],
};

pub const LOWER_N: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 6 },
        DeltaPosition { dx: 2, dy: 9 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 6, dy: 6 },
        DeltaPosition { dx: 6, dy: 9 },
    ],
};

pub const LOWER_O: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 8 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 8 },
    ],
};

pub const LOWER_P: ActionLetter = ActionLetter {
    width: 8,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 8 },
        DeltaPosition { dx: 2, dy: 13 },
        DeltaPosition { dx: 4, dy: 6 },
        DeltaPosition { dx: 4, dy: 11 },
        DeltaPosition { dx: 6, dy: 8 },
    ],
};

pub const LOWER_R: ActionLetter = ActionLetter {
    width: 7,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 6 },
        DeltaPosition { dx: 2, dy: 11 },
        DeltaPosition { dx: 4, dy: 6 },
    ],
};

pub const LOWER_W: ActionLetter = ActionLetter {
    width: 9,

    // Delta from the top left corner of the letters box.
    checkpoints: &[
        DeltaPosition { dx: 2, dy: 6 },
        DeltaPosition { dx: 2, dy: 9 },
        DeltaPosition { dx: 5, dy: 8 },
        DeltaPosition { dx: 7, dy: 6 },
        DeltaPosition { dx: 7, dy: 9 },
    ],
};

pub const SPACE: ActionLetter = ActionLetter {
    width: 5,

    // Delta from the top left corner of the letters box.
    checkpoints: &[],
};

/// Position which should be inactive in the top bar of the game window. Can be
/// clicked to make sure that the game window is in focus.
pub const TOP_BAR: Position = Position { x: 1500, y: 40 };

/// Inventory button. Used to check if the inventory is open or not.
pub const INVENTORY_BUTTON: Position = Position { x: 1594, y: 628 };
pub const INVENTORY_OPEN: FuzzyPixel = FuzzyPixel {
    blue_min: 28,
    blue_max: 30,
    green_min: 37,
    green_max: 39,
    red_min: 112,
    red_max: 114,
};

/// Chat buttons. Need to check them to make sure the chat box is closed.
pub const ALL_CHAT_BUTTON: Position = Position { x: 975, y: 645 };
pub const ALL_CHAT_ON_HIGHLIGHT: FuzzyPixel = FuzzyPixel {
    blue_min: 39,
    blue_max: 42,
    green_min: 49,
    green_max: 52,
    red_min: 58,
    red_max: 61,
};

pub const CHAT_BOX_TOP_LEFT: (Position, FuzzyPixel) = (
    Position { x: 970, y: 497 },
    FuzzyPixel {
        blue_min: 114,
        blue_max: 114,
        green_min: 137,
        green_max: 137,
        red_min: 147,
        red_max: 147,
    },
);
pub const CHAT_BOX_BOTTOM_LEFT: (Position, FuzzyPixel) = (
    Position { x: 965, y: 630 },
    FuzzyPixel {
        blue_min: 147,
        blue_max: 147,
        green_min: 169,
        green_max: 169,
        red_min: 173,
        red_max: 173,
    },
);

pub const CHAT_BOX_TOP_RIGHT: (Position, FuzzyPixel) = (
    Position { x: 1478, y: 499 },
    FuzzyPixel {
        blue_min: 94,
        blue_max: 94,
        green_min: 112,
        green_max: 112,
        red_min: 119,
        red_max: 119,
    },
);

pub const CHAT_BOX_BOTTOM_RIGHT: (Position, FuzzyPixel) = (
    Position { x: 1480, y: 630 },
    FuzzyPixel {
        blue_min: 140,
        blue_max: 140,
        green_min: 154,
        green_max: 154,
        red_min: 162,
        red_max: 162,
    },
);

/// When selecting an action, the name of the object to act upon appears in the
/// top left in electric blue.
pub const OBJECT_NAME_BLUE: FuzzyPixel = FuzzyPixel {
    blue_min: 221,
    blue_max: 221,
    green_min: 221,
    green_max: 221,
    red_min: 0,
    red_max: 0,
};

pub const ACTION_DESCRIPTION_Y_MAX: i32 = 70;

pub const CHARACTER_WIDTH: i32 = 7;

pub const CHOP_DOWN_TREE_BOUNDS: BoundingBox =
    BoundingBox(Position { x: 967, y: 40 }, Position { x: 1070, y: 70 });

/// Pixels to check that are in the shape of the word Tree to confirm that object we hover over is in fact a tree for us to chop down.
pub const CHOP_DOWN_ACTION_OUTLINE: &[Position] = &[];
pub const TREE_ACTION_OUTLINE: &[Position] = &[];
