// Each pixel is represented by 4 u8's, BGRA/RGBA. Each frame is a list of u8's.
pub const RAW_PIXEL_SIZE: usize = 4;

pub mod colors {
    use crate::{FuzzyPixel, Pixel};

    pub const PURE_RED: Pixel = Pixel {
        red: 255,
        green: 0,
        blue: 0,
    };

    pub const CHATBOX_INNER_TOP_LEFT: FuzzyPixel = FuzzyPixel {
        blue_min: 64,
        blue_max: 70,
        green_min: 78,
        green_max: 84,
        red_min: 85,
        red_max: 92,
    };
    pub const CHATBOX_INNER_BOTTOM_LEFT: FuzzyPixel = FuzzyPixel {
        blue_min: 82,
        blue_max: 89,
        green_min: 100,
        green_max: 108,
        red_min: 109,
        red_max: 118,
    };
    pub const CHATBOX_INNER_TOP_RIGHT: FuzzyPixel = FuzzyPixel {
        blue_min: 115,
        blue_max: 119,
        green_min: 143,
        green_max: 147,
        red_min: 155,
        red_max: 159,
    };
    pub const CHATBOX_INNER_BOTTOM_RIGHT: FuzzyPixel = FuzzyPixel {
        blue_min: 115,
        blue_max: 119,
        green_min: 143,
        green_max: 147,
        red_min: 155,
        red_max: 159,
    };

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

    pub const ACTION_YELLOW: FuzzyPixel = FuzzyPixel {
        blue_min: 0,
        blue_max: 30,
        green_min: 190,
        green_max: 235,
        red_min: 190,
        red_max: 235,
    };

    pub const WHITE: FuzzyPixel = FuzzyPixel {
        blue_min: 255,
        blue_max: 255,
        green_min: 255,
        green_max: 255,
        red_min: 255,
        red_max: 255,
    };

    // Used to make a dollar sign to mark a bank on the mini/worldmap.
    pub const BANK_ICON_YELLOW: FuzzyPixel = FuzzyPixel {
        blue_min: 74,
        blue_max: 78,
        green_min: 207,
        green_max: 211,
        red_min: 230,
        red_max: 234,
    };

    // Far from the picture within the icon, outside of its shadow.
    pub const MAP_ICON_LIGHT_GRAY: FuzzyPixel = FuzzyPixel {
        blue_min: 190,
        blue_max: 194,
        green_min: 189,
        green_max: 193,
        red_min: 189,
        red_max: 193,
    };
    pub const MAP_ICON_DARK_GRAY: FuzzyPixel = FuzzyPixel {
        blue_min: 157,
        blue_max: 161,
        green_min: 158,
        green_max: 162,
        red_min: 158,
        red_max: 162,
    };

    // TODO: Change to active background.
    pub const INVENTORY_BACKGROUND: FuzzyPixel = FuzzyPixel {
        blue_min: 37,
        blue_max: 46,
        green_min: 49,
        green_max: 57,
        red_min: 58,
        red_max: 65,
    };

    /// This is the red color of the inventory icon when the inventory is open.
    /// Corresponds to the location of INVENTORY_ICON_BACKGROUND.
    pub const INVENTORY_ICON_BACKGROUND_OPEN: FuzzyPixel = FuzzyPixel {
        blue_min: 25,
        blue_max: 35,
        green_min: 35,
        green_max: 45,
        red_min: 110,
        red_max: 130,
    };

    pub const TREE_BARK: FuzzyPixel = FuzzyPixel {
        blue_min: 40,
        blue_max: 44,
        green_min: 81,
        green_max: 85,
        red_min: 114,
        red_max: 118,
    };

    pub const OAK_BARK: FuzzyPixel = FuzzyPixel {
        blue_min: 40,
        blue_max: 44,
        green_min: 81,
        green_max: 85,
        red_min: 114,
        red_max: 118,
    };

    pub const SMALL_NET_FISHING_SPOT: FuzzyPixel = FuzzyPixel {
        blue_min: 105,
        blue_max: 115,
        green_min: 115,
        green_max: 135,
        red_min: 140,
        red_max: 155,
    };
}
