use util::*;

// TODO: refactor away from simple, compound, abstract. Move to just grouping by
// topic: Bank, map, etc.

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

    pub fn new(top_left: Position, dimensions: DeltaPosition) -> Locations {
        // We are tied to the assumption that the screen is wide enough for all
        // the icons in the bottom right to fit in 1 row.
        assert!(dimensions.dx > 947);

        Locations {
            top_left,
            dimensions,
        }
    }

    pub fn to_bottom_left(top_left: Position, dimensions: DeltaPosition) -> Position {
        Position {
            x: top_left.x,
            y: top_left.y + dimensions.dy - 1,
        }
    }
    pub fn to_top_right(top_left: Position, dimensions: DeltaPosition) -> Position {
        Position {
            x: top_left.x + dimensions.dx - 1,
            y: top_left.y,
        }
    }
    pub fn to_bottom_right(top_left: Position, dimensions: DeltaPosition) -> Position {
        Position {
            x: top_left.x + dimensions.dx - 1,
            y: top_left.y + dimensions.dy - 1,
        }
    }

    // Corners of the screen.
    fn bottom_left(&self) -> Position {
        Self::to_bottom_left(self.top_left, self.dimensions)
    }

    fn top_right(&self) -> Position {
        Self::to_top_right(self.top_left, self.dimensions)
    }

    fn bottom_right(&self) -> Position {
        Self::to_bottom_right(self.top_left, self.dimensions)
    }

    // Locations given in reference to the top left corner of the screen.
    pub fn action_text_top_left(&self) -> Position {
        Position {
            x: self.top_left.x + 5,
            y: self.top_left.y + 3,
        }
    }
    // A specific point near the edge of an enemy healthbar.
    pub fn enemy_healthbar_left(&self) -> Position {
        Position {
            x: self.top_left.x + 8,
            y: self.top_left.y + 45,
        }
    }
    pub fn enemy_healthbar_right(&self) -> Position {
        Position {
            x: self.top_left.x + 130,
            y: self.top_left.y + 45,
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
    const WORLDMAP_OUTER_BORDER_WIDTH: i32 = 6;
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
            dx: self.minimap_plus_top_left().x - x0 - 4 - Self::WORLDMAP_OUTER_BORDER_WIDTH,
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
        // TODO: move from constant size to constant number, which will scale
        // with screen size.
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

    pub fn smith_box_dimensions(&self) -> DeltaPosition {
        // Size of the pop up box when smithing at an anvil.
        DeltaPosition { dx: 488, dy: 308 }
    }
    pub fn smith_box_top_left(&self) -> Position {
        let top_left_y_offset =
            (self.chatbox_outer_top_left().y - self.top_left.y - self.smith_box_dimensions().dy)
                / 2;
        let top_left_x_offset =
            (self.minimap_plus_top_left().x - 4 - self.top_left.x - self.smith_box_dimensions().dx)
                / 2;
        Position {
            x: self.top_left.x + top_left_x_offset,
            y: self.top_left.y + top_left_y_offset,
        }
    }
    pub fn smith_box_platelegs(&self) -> Position {
        let Position { x, y } = self.smith_box_top_left();
        Position {
            x: x + 185,
            y: y + 100,
        }
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
    pub fn chatbox_middle(&self) -> Position {
        Self::midpoint(
            self.chatbox_inner_top_left(),
            self.chatbox_inner_dimensions(),
        )
    }

    /// Locations for understanding the bank. This is for the bank being open,
    /// not navigating around the location of a bank.
    ///
    /// The bank is surrounded by a border (like the worldmap) which seems to be
    /// constant width on all sides. We will give locations internally, not to
    /// the outside edge.
    ///
    /// The bank has 2 forms of symmetry.
    /// - Vertically it is symmetric between the top of the screen and the
    ///   chatbox. It extends to fill this space up until a certain max height,
    ///   and then space is added symmetrically above and below.
    /// - Horizontally the bank is centered around the center of the worldmap.
    pub const BANK_BORDER_WIDTH: i32 = 6;
    pub const NUM_BANK_COLUMNS: i32 = 8;
    /// Maximum number of rows that we can assume to have access to. This number
    /// can be much larger if the screen is expanded, but the bot will only use
    /// the first 5.
    pub const NUM_BANK_ROWS: i32 = 5;
    pub const NUM_BANK_SLOTS: i32 = Self::NUM_BANK_COLUMNS * Self::NUM_BANK_ROWS;
    /// The bank expands to fill up more vertical space until the inside area is
    /// 788 pixels high.
    pub const BANK_MAX_HEIGHT: i32 = 788;

    /// This gets the vertical distance from either the top of the screen or the
    /// top of te chatbox to the first internal pixel of the bank (within the
    /// border).
    fn bank_top_offset(&self) -> i32 {
        // There is always at least 2 pixels between the top of the screen and
        // the top of the bank border. Below the screen there is always at least
        // 1 pixel between the top of the chatbox and the bottom of the bank
        // border.
        let min_vertical_offset = 3 + 2 * Self::BANK_BORDER_WIDTH;
        let total_vertical_space = self.chatbox_outer_top_left().y - self.top_left.y;

        let height = total_vertical_space - min_vertical_offset;
        if height > Self::BANK_MAX_HEIGHT {
            ((total_vertical_space - Self::BANK_MAX_HEIGHT) as f32 / 2.0).round() as i32
        } else {
            2 + Self::BANK_BORDER_WIDTH
        }
    }
    // The bank box extends from the top of the screen until the top of the
    // chatbox up until a max height. Then space is added above and below it
    // symmetrically between the top of the screen and the top of the
    // chatbox.
    pub fn bank_dimensions(&self) -> DeltaPosition {
        // The spacing above the bank is 1 pixel larger than the spacing below the bank.
        let vertical_spacing = 2 * self.bank_top_offset() - 1;
        let total_vertical_space = self.chatbox_outer_top_left().y - self.top_left.y;
        DeltaPosition {
            dx: 476,
            dy: total_vertical_space - vertical_spacing,
        }
    }
    pub fn bank_top_left(&self) -> Position {
        // Due to a difference in rounding centering the bank works better when
        // we subtract 1 from the worldmap width.
        let DeltaPosition { dx, dy } = self.worldmap_dimensions();
        let Position { x, y: _y } =
            Self::midpoint(self.worldmap_top_left(), DeltaPosition { dx: dx - 1, dy });
        Position {
            x: x - (self.bank_dimensions().dx as f32 / 2.0).round() as i32,
            y: self.top_left.y + self.bank_top_offset(),
        }
    }
    pub fn bank_slot_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 48, dy: 36 }
    }
    /// Only make use of the bank slot center. This is different than the
    /// inventory which uses top_left and dimensions so that we can analyze the
    /// contents. This is because I am a bit less confident my bank locations
    /// scale perfectly and the numbers for items messes with the analysis. I
    /// don't think it will be critical for me to analyze items and instead I
    /// can just give an explicit slot_index for an item.
    pub fn bank_slot_center(&self, slot_index: i32) -> Position {
        const BANK_TITLE_BAR_HEIGHT: i32 = 23;
        const BANK_TABS_HEIGHT: i32 = 40;
        const BANK_INCINERATOR_WIDTH: i32 = 46;

        assert!(slot_index < Self::NUM_BANK_SLOTS);
        let row = slot_index / Self::NUM_BANK_COLUMNS;
        let col = slot_index - row * Self::NUM_BANK_COLUMNS;

        let Position { x, y } = self.bank_top_left();
        let DeltaPosition { dx, dy } = self.bank_slot_dimensions();

        // There is a border of open space in the inventory where items are
        // never put. So there is an offset from the top left corner of the
        // inventory to where teh first slot is placed.
        let x0 = x + BANK_INCINERATOR_WIDTH;
        let y0 = y + BANK_TITLE_BAR_HEIGHT + Self::BANK_BORDER_WIDTH + BANK_TABS_HEIGHT;
        Position {
            x: x0 + col * dx + dx / 2,
            y: y0 + row * dy + dy / 2,
        }
    }
    pub fn bank_deposit_inventory(&self) -> Position {
        let Position { x, y } = Self::to_bottom_right(self.bank_top_left(), self.bank_dimensions());
        Position {
            x: x - 50,
            y: y - 15,
        }
    }
    pub fn bank_quantity_all(&self) -> Position {
        let Position { x, y } = Self::to_bottom_right(self.bank_top_left(), self.bank_dimensions());
        Position {
            x: x - 166,
            y: y - 5,
        }
    }
    pub fn bank_quantity_x(&self) -> Position {
        let Position { x, y } = Self::to_bottom_right(self.bank_top_left(), self.bank_dimensions());
        Position {
            x: x - 191,
            y: y - 10,
        }
    }
    pub fn bank_quantity_one(&self) -> Position {
        let Position { x, y } = Self::to_bottom_right(self.bank_top_left(), self.bank_dimensions());
        Position {
            x: x - 267,
            y: y - 10,
        }
    }

    // Locations given in reference to the top right corner of the screen.

    /// Minimap Plus top left is used to create a box around the mini map and
    /// the icons around it such as health and compass etc.
    pub fn minimap_plus_top_left(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position { x: x - 210, y }
    }
    pub fn minimap_plus_dimensions(&self) -> DeltaPosition {
        DeltaPosition { dx: 211, dy: 173 }
    }
    /// The minimap is circular, so we analyze it using polar coordinates, middle & radius.
    pub fn minimap_middle(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position {
            x: x - 82,
            y: y + 84,
        }
    }
    /// The minimap radius is to the beginning of the green & blue part of the
    /// worldmap icon. This is to avoid an issue of looking for a blue/green and
    /// accidentally clicking the worldmap.
    pub const MINIMAP_RADIUS: i32 = 72;
    pub const MINIMAP_SMALL_RADIUS: i32 = Self::MINIMAP_RADIUS / 6;
    /// When we find something interesting in the minimap we often want to check
    /// the adjacent pixels to confirm this is not an abberant pixel. Should be
    /// about the same as the radius of an icon on the map.
    pub const CHECK_ADJACENT_MAP_PIXELS_RADIUS: i32 = 8;

    pub const WROLDMAP_ICON_RADIUS: i32 = 12;
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
    pub fn run_icon(&self) -> Position {
        let Position { x, y } = self.top_right();
        Position {
            x: x - 159,
            y: y + 132,
        }
    }

    // Locations given in reference to the bottom right corner of the screen.
    const NUM_INVENTORY_ROWS: i32 = 7;
    const NUM_INVENTORY_COLS: i32 = 4;
    pub const NUM_INVENTORY_SLOTS: i32 = Self::NUM_INVENTORY_ROWS * Self::NUM_INVENTORY_COLS;
    // Spacing between pixels to check in a slot. This value must remain
    // constant because it is the basis for checking if items in the inventory
    // match and changing the spacing here would change where pixels are checked
    // and teh total number of checks.
    pub const INVENTORY_SLOT_CHECK_SPACING: DeltaPosition = DeltaPosition { dx: 9, dy: 9 };
    // Based on INVENTORY_SLOT_CHECK_SPACING and the size of each inventory
    // slot, which is seems to be constant across different screen sizes, this
    // is the number of pixels checked per inventory slot. This is truly
    // expected to remain constant since we tie this to the recipes passed to
    // FrameHandler::check_inventory_slot.
    pub const NUM_CHECKS_PER_INVENTORY_SLOT: usize = 12;

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
        assert!(slot_index < Self::NUM_INVENTORY_SLOTS);
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
    pub fn inventory_slot_middle(&self, slot_index: i32) -> Position {
        Self::midpoint(
            self.inventory_slot_top_left(slot_index),
            self.inventory_slot_dimensions(),
        )
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
    pub fn open_screen_dimensions(&self) -> DeltaPosition {
        DeltaPosition {
            // We usually play with the inventory open so only search as far right
            // as either the inventory or minimap extends left.
            dx: std::cmp::min(
                self.inventory_outer_top_left().x,
                self.minimap_plus_top_left().x,
            ) - self.top_left.x
                + 1,
            // We assume that the chatbox is closed in which case the icons on the
            // bottom extend up higher than the chat buttons.
            dy: self.leftmost_bottom_icon_top_left().y - self.top_left.y + 1,
        }
    }
    pub fn open_screen_search_boxes(&self) -> Vec<(Position, DeltaPosition)> {
        let top_left = self.top_left;
        let dimensions = self.open_screen_dimensions();
        let past_bottom_right = top_left + dimensions;

        let mut ret = Vec::<(Position, DeltaPosition)>::new();
        let mut box_top_left = self.mid_screen();

        // TODO: move from constant size to constant number, which will scale
        // with screen size.
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
