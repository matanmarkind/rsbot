use util::*;

/// Absolute values describing the screen's. Most values should be given in
/// reference to this point. Changing this will move every other "const fn"
/// listed.
pub const WINDOW_TOP_LEFT: Position = Position { x: 960, y: 26 };
pub const WINDOW_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 960, dy: 637 };

// Between the window for the program and the game screen there is a border. At
// the top this has options such minimize, and on the sides and bottom it is
// simply padding.
pub const SIDE_BORDER_WIDTH: i32 = 4;
pub const BOTTOM_BORDER_HEIGHT: i32 = 5;
pub const TOP_BAR_HEIGHT: i32 = 27;

/// All other locations are defined in terms of where they are in reference to
/// WINDOW_TOP_LEFT.
pub const SCREEN_TOP_LEFT: Position = Position {
    x: WINDOW_TOP_LEFT.x + SIDE_BORDER_WIDTH,
    y: WINDOW_TOP_LEFT.y + TOP_BAR_HEIGHT,
};

/// When the mouse hovers over an object, the action that would be taken by left
/// clicking is displayed in the top left corner.
pub const TOP_LEFT_ACTION_TEXT: Position = Position {
    x: WINDOW_TOP_LEFT.x + 9,
    y: WINDOW_TOP_LEFT.y + 31,
};

/// Location used to click to make sure that the rs window is the active window.
pub const TOP_BAR_MIDDLE: Position = Position {
    x: WINDOW_TOP_LEFT.x + WINDOW_DIMENSIONS.dx / 2,
    y: WINDOW_TOP_LEFT.y + TOP_BAR_HEIGHT / 2,
};

/// Middle of the minimap is where player dot is located. Pressing here should
/// not cause us to move.
pub const MINIMAP_MIDDLE: Position = Position {
    x: WINDOW_TOP_LEFT.x + WINDOW_DIMENSIONS.dx - 85,
    y: WINDOW_TOP_LEFT.y + 113,
};

/// This is part of the gray background for the inventory icon in the bottom of
/// the screen. Used to tell if the inventory is open or closed.
pub const INVENTORY_ICON: Position = Position {
    x: WINDOW_TOP_LEFT.x + 632,
    y: WINDOW_TOP_LEFT.y + 606,
};
