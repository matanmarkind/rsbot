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

/// Box in the middle (ish) of the screen. Used to search for things closer to the
/// player first.
pub const NEARBY_SCREEN_TOP_LEFT: Position = Position {
    x: WINDOW_TOP_LEFT.x + 100,
    y: WINDOW_TOP_LEFT.y + 224,
};
pub const NEARBY_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 650, dy: 350 };
pub const VERY_NEARBY_SCREEN_TOP_LEFT: Position = Position {
    x: WINDOW_TOP_LEFT.x + 250,
    y: WINDOW_TOP_LEFT.y + 224,
};
pub const VERY_NEARBY_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 350, dy: 250 };

/// The area of the screen that should be open for searching for things in
/// the world. This is when the chatbox is closed, and the inventory is
/// open. With the minimap the right colum of the screen is basically out of
/// commision.
pub const OPEN_SCREEN_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 750, dy: 570 };

/// Middle of the minimap is where player dot is located. Pressing here should
/// not cause us to move.
pub const MINIMAP_MIDDLE: Position = Position {
    x: WINDOW_TOP_LEFT.x + WINDOW_DIMENSIONS.dx - 85,
    y: WINDOW_TOP_LEFT.y + 113,
};

/// This is part of the gray background for the inventory icon in the bottom of
/// the screen. Used to tell if the inventory is open or closed.
pub const INVENTORY_ICON_BACKGROUND: Position = Position {
    x: WINDOW_TOP_LEFT.x + 632,
    y: WINDOW_TOP_LEFT.y + 606,
};
/// Interior bounds of the inventory itself.
pub const INVENTORY_TOP_LEFT: Position = Position {
    x: WINDOW_TOP_LEFT.x + 759,
    y: WINDOW_TOP_LEFT.y + 330,
};
pub const INVENTORY_BOTTOM_RIGHT: Position = Position {
    x: WINDOW_TOP_LEFT.x + 948,
    y: WINDOW_TOP_LEFT.y + 590,
};
pub const INVENTORY_FIRST_SLOT: Position = Position {
    x: INVENTORY_TOP_LEFT.x + 10,
    y: INVENTORY_TOP_LEFT.y + 6,
};
pub const INVENTORY_SLOT_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 42, dy: 36 };

/// Used to calculate the pixel location of a desired object on the screen.
///
/// For interesting locations that don't move, we have recorded their location
/// in Fullscreen mode and can scale those to the correct location.
pub struct Locations {
    // Locations are given in reference to the gameplay screen (as opposed to
    // the process window. This is because window attributes will likely
    // change across computers and also side buffers change depending on
    // fullscreen mode)
    pub top_left: Position,

    // Width and height of the screen, such that the bottom right pixel is
    // (top_left + dimensions - 1).
    pub dimensions: DeltaPosition,
}

impl Locations {
    // Icons at the bottom right of the screen seem to keep the same dimensions
    // even as the screen stretches.
    pub const BOTTOM_ICONS_DIMENSIONS: DeltaPosition = DeltaPosition { dx: 33, dy: 35 };
    pub const NUM_BOTTOM_ICONS: i32 = 13;
    const WORLDMAP_OUTER_BORDER_WIDTH: i32 = 6;

    pub fn new(top_left: Position, dimensions: DeltaPosition) -> Locations {
        // We are tied to the assumption that the screen is wide enough for all
        // the icons in the bottom right to fit in 1 row.
        assert!(dimensions.dx > 947);

        Locations {
            top_left,
            dimensions,
        }
    }

    // Corners of the screen.
    fn bottom_left(&self) -> Position {
        Position {
            x: self.top_left.x,
            y: self.top_left.y + self.dimensions.dy - 1,
        }
    }

    fn top_right(&self) -> Position {
        Position {
            x: self.top_left.x + self.dimensions.dx - 1,
            y: self.top_left.y,
        }
    }

    fn bottom_right(&self) -> Position {
        Position {
            x: self.top_left.x + self.dimensions.dx - 1,
            y: self.top_left.y + self.dimensions.dy - 1,
        }
    }

    // Locations given in reference to the top left corner of the screen.
    pub fn action_text_top_left(&self) -> Position {
        Position {
            x: self.top_left.x + 4,
            y: self.top_left.y + 3,
        }
    }
    pub fn midpoint(top_left: Position, dimensions: DeltaPosition) -> Position {
        Position {
            x: top_left.x + (dimensions.dx as f32 / 2.0).round() as i32,
            y: top_left.y + (dimensions.dy as f32 / 2.0).round() as i32,
        }
    }
    pub fn mid_screen(&self) -> Position {
        Self::midpoint(self.top_left, self.dimensions)
    }
    // For worldmap we give only innter positions/dimensions.
    pub fn worldmap_top_left(&self) -> Position {
        Position {
            x: self.top_left.x + Self::WORLDMAP_OUTER_BORDER_WIDTH,
            y: self.top_left.y + Self::WORLDMAP_OUTER_BORDER_WIDTH,
        }
    }
    pub fn worldmap_dimensions(&self) -> DeltaPosition {
        // The worldap extends from the top left of the screen until the chatbox
        // at the bottom and near the mini map area on the right. To convert
        // this to dimensions we calculate the distance from the worldmap's
        // top_left to these points and then subtract out any padding.
        let Position { x: x0, y: y0 } = self.worldmap_top_left();
        DeltaPosition {
            // There is a 5 pixel space between the worldmaps border ending and
            // the minimap box starting.
            dx: self.minimap_top_left().x - x0 - 4 - Self::WORLDMAP_OUTER_BORDER_WIDTH,
            dy: self.chatbox_outer_top_left().y - y0 - Self::WORLDMAP_OUTER_BORDER_WIDTH,
        }
    }
    fn worldmap_key_dimensions(&self) -> DeltaPosition {
        let DeltaPosition { dx: _, dy } = self.worldmap_dimensions();
        DeltaPosition {
            dx: 168,
            dy: dy - Self::WORLDMAP_OUTER_BORDER_WIDTH - self.worldmap_bottom_bar_dimensions().dy,
        }
    }
    fn worldmap_bottom_bar_dimensions(&self) -> DeltaPosition {
        let DeltaPosition { dx, dy: _ } = self.worldmap_dimensions();
        DeltaPosition { dx, dy: 32 }
    }
    pub fn worldmap_map_top_left(&self) -> Position {
        let Position { x, y } = self.worldmap_top_left();
        Position {
            x: x + self.worldmap_key_dimensions().dx - 1,
            y,
        }
    }
    pub fn worldmap_map_dimensions(&self) -> DeltaPosition {
        let DeltaPosition { dx, dy } = self.worldmap_dimensions();
        DeltaPosition {
            dx: dx - self.worldmap_key_dimensions().dx + 1,
            dy: dy - self.worldmap_bottom_bar_dimensions().dy - Self::WORLDMAP_OUTER_BORDER_WIDTH,
        }
    }
    pub fn worldmap_map_middle(&self) -> Position {
        Self::midpoint(self.worldmap_map_top_left(), self.worldmap_map_dimensions())
    }
    pub fn worldmap_map_search_boxes(&self) -> Vec<(Position, DeltaPosition)> {
        let top_left = self.worldmap_map_top_left();
        let dimensions = self.worldmap_map_dimensions();

        let mut ret = Vec::<(Position, DeltaPosition)>::new();
        let mut box_top_left = self.worldmap_map_middle();

        // Based on the angle the camera is from, things lower in the screen
        // tend to be closer to the player than things higher in the screen.
        // Therefore prefer widening the search more down the screen than up the
        // screen.
        let step_size = 50;
        let mut box_dimensions = DeltaPosition { dx: 0, dy: 0 };
        loop {
            box_top_left = Position {
                x: box_top_left.x - step_size,
                y: box_top_left.y - step_size,
            };
            if box_top_left.x <= top_left.x || box_top_left.y <= top_left.y {
                // Once we reach an edge just make the last box of the entire screen.
                ret.push((top_left, dimensions));
                break;
            }

            box_dimensions = DeltaPosition {
                dx: box_dimensions.dx + 2 * step_size,
                dy: box_dimensions.dy + 2 * step_size,
            };
            ret.push((box_top_left, box_dimensions));
        }
        ret
    }

    // Locations given in reference to the bottom left corner of the screen.
    pub fn all_chat_button(&self) -> Position {
        let Position { x, y } = self.bottom_left();
        Position {
            x: x + 17,
            y: y - 12,
        }
    }
    pub fn chatbox_outer_top_left(&self) -> Position {
        let Position { x, y } = self.bottom_left();
        Position { x: x, y: y - 164 }
    }
    pub fn chatbox_outer_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 519, dy: 142 }
    }
    pub fn chatbox_inner_top_left(&self) -> Position {
        let Position { x, y } = self.bottom_left();
        Position {
            x: x + 7,
            y: y - 157,
        }
    }
    pub fn chatbox_inner_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 506, dy: 129 }
    }

    // Locations given in reference to the top right corner of the screen.

    /// Minimap top left is used to create a box around the mini map and the
    /// icons around it such as health and compass etc.
    pub fn minimap_top_left(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position { x: x - 210, y }
    }
    pub fn minimap_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 211, dy: 173 }
    }
    pub fn minimap_middle(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position {
            x: x - 80,
            y: y + 84,
        }
    }
    pub fn worldmap_icon(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position {
            x: x - 19,
            y: y + 140,
        }
    }
    pub fn compass_icon(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position {
            x: x - 158,
            y: y + 23,
        }
    }

    // Locations given in reference to the bottom right corner of the screen.
    const NUM_INVENTORY_ROWS: i32 = 7;
    const NUM_INVENTORY_COLS: i32 = 4;
    pub const NUM_INVENTORY_SLOTS: i32 = Self::NUM_INVENTORY_ROWS * Self::NUM_INVENTORY_COLS;
    // Spacing between pixels to check in a slot.
    pub const INVENTORY_SLOT_CHECK_SPACING: DeltaPosition = DeltaPosition { dx: 9, dy: 9 };

    pub fn inventory_outer_top_left(&self) -> Position {
        let Position { x, y } = self.bottom_right();
        Position {
            x: x - 202,
            y: y - 309,
        }
    }
    pub fn inventory_outer_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 202, dy: 273 }
    }
    pub fn inventory_inner_top_left(&self) -> Position {
        let Position { x, y } = self.bottom_right();
        Position {
            x: x - 197,
            y: y - 304,
        }
    }
    pub fn inventory_inner_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 191, dy: 262 }
    }
    pub fn inventory_slot_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 42, dy: 36 }
    }
    pub fn inventory_slot_top_left(&self, slot_index: i32) -> Position {
        let row = slot_index / Self::NUM_INVENTORY_COLS;
        let col = slot_index - row * Self::NUM_INVENTORY_COLS;

        let Position { x, y } = self.inventory_inner_top_left();
        let DeltaPosition { dx, dy } = self.inventory_slot_dimensions();

        // There is a border of open space in the inventory where items are
        // never put. So there is an offset from the top left corner of the
        // inventory to where teh first slot is placed.
        Position {
            x: x + 11 + col * dx,
            y: y + 8 + row * dy,
        }
    }

    // At the bottom of the screen, to the right of the chat icons are icons for
    // many different features with menus (inventory, combat, etc.). This
    // assumes the screen is wide enough to show all of these icons in 1 row.
    fn leftmost_bottom_icon_top_left(&self) -> Position {
        let Position { x, y } = self.bottom_right();
        Position {
            x: x - 428,
            y: y - 35,
        }
    }
    // icon_index is 0 indexed (starting at combat).
    fn bottom_icon_top_left(&self, icon_index: i32) -> Position {
        let Position { x, y } = self.leftmost_bottom_icon_top_left();
        Position {
            x: x + icon_index * Self::BOTTOM_ICONS_DIMENSIONS.dx,
            y,
        }
    }
    // An offset of (4, 4) seems to give a consistent pixel color identifying
    // when an icon is active/passive.
    fn bottom_icon_background(&self, icon_index: i32) -> Position {
        assert!(icon_index < Self::NUM_BOTTOM_ICONS);
        let Position { x, y } = self.bottom_icon_top_left(icon_index);
        Position { x: x + 4, y: y + 4 }
    }
    pub fn inventory_icon_background(&self) -> Position {
        self.bottom_icon_background(3)
    }

    // Create boxes used for searching for things in the open screen.
    pub fn open_screen_search_boxes(&self) -> Vec<(Position, DeltaPosition)> {
        let top_left = self.top_left;
        let past_bottom_right = Position {
            // We usually play with the inventory open so only search as far right
            // as either the inventory or minimap extends left.
            x: std::cmp::min(self.inventory_outer_top_left().x, self.minimap_top_left().x) + 1,
            // We assume that the chatbox is closed in which case the icons on the
            // bottom extend up higher than the chat buttons.
            y: self.leftmost_bottom_icon_top_left().y + 1,
        };
        let dimensions = past_bottom_right - top_left;

        let mut ret = Vec::<(Position, DeltaPosition)>::new();
        let mut box_top_left = self.mid_screen();

        let step_size = 50;
        let mut box_dimensions = DeltaPosition { dx: 0, dy: 0 };
        loop {
            // Based on the angle the camera is from, things lower in the screen
            // tend to be closer to the player than things higher in the screen.
            // Therefore prefer widening the search more down the screen than up the
            // screen.
            box_top_left = Position {
                x: box_top_left.x - 2 * step_size,
                y: box_top_left.y - step_size,
            };
            if box_top_left.x <= top_left.x || box_top_left.y <= top_left.y {
                // Once we reach an edge just make the last box of the entire screen.
                ret.push((top_left, dimensions));
                break;
            }

            box_dimensions = DeltaPosition {
                dx: std::cmp::min(
                    box_dimensions.dx + 4 * step_size,
                    past_bottom_right.x - box_top_left.x,
                ),
                dy: std::cmp::min(
                    box_dimensions.dy + 3 * step_size,
                    past_bottom_right.y - box_top_left.y,
                ),
            };
            ret.push((box_top_left, box_dimensions));
        }
        ret
    }
}
