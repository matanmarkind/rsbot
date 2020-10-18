// Each pixel is represented by 4 u8's, BGRA/RGBA. Each frame is a list of u8's.
pub const RAW_PIXEL_SIZE: usize = 4;

/// TODO: Break this into multiple modules. pixels, fuzzy_pixels,
/// inventory_slot_pixels.
pub mod colors {
    use crate::InventorySlotPixels;
    use crate::Locations;
    use crate::{FuzzyPixel, Pixel};

    // Pixels.
    pub const PURE_RED: Pixel = Pixel {
        red: 255,
        green: 0,
        blue: 0,
    };

    // FuzzyPixels
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

    pub const BANK_BROWN1: FuzzyPixel = FuzzyPixel {
        blue_min: 1,
        blue_max: 5,
        green_min: 30,
        green_max: 40,
        red_min: 50,
        red_max: 60,
    };
    pub const BANK_BROWN2: FuzzyPixel = FuzzyPixel {
        blue_min: 1,
        blue_max: 5,
        green_min: 40,
        green_max: 50,
        red_min: 61,
        red_max: 71,
    };
    pub const BANK_BROWN3: FuzzyPixel = FuzzyPixel {
        blue_min: 20,
        blue_max: 24,
        green_min: 62,
        green_max: 66,
        red_min: 85,
        red_max: 89,
    };

    /// Colors for icons that show up on the maps (seems to be the same worldmap
    /// and minimap.)
    // Used to make a dollar sign to mark a bank on the mini/worldmap.
    pub const MAP_ICON_BANK_YELLOW: FuzzyPixel = FuzzyPixel {
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

    // Fires seem to be dominated by 2 shades. Since they are fairly distinct
    // they should be searched seperately.
    pub const FIRE_OPAQUE: FuzzyPixel = FuzzyPixel {
        blue_min: 25,
        blue_max: 45,
        green_min: 160,
        green_max: 190,
        red_min: 220,
        red_max: 255,
    };
    pub const FIRE_TRANSLUCENT: FuzzyPixel = FuzzyPixel {
        blue_min: 25,
        blue_max: 45,
        green_min: 125,
        green_max: 145,
        red_min: 180,
        red_max: 190,
    };

    // Pixel arrays for matching items in the inventory.
    pub const INVENTORY_SLOT_EMPTY: InventorySlotPixels =
        [INVENTORY_BACKGROUND; Locations::NUM_CHECKS_PER_INVENTORY_SLOT];

    pub const INVENTORY_RAW_SHRIMP: InventorySlotPixels = [
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 105,
            blue_max: 109,
            green_min: 119,
            green_max: 123,
            red_min: 145,
            red_max: 149,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 0,
            blue_max: 3,
            green_min: 0,
            green_max: 2,
            red_min: 0,
            red_max: 2,
        },
        FuzzyPixel {
            blue_min: 35,
            blue_max: 39,
            green_min: 50,
            green_max: 54,
            red_min: 78,
            red_max: 82,
        },
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 99,
            blue_max: 103,
            green_min: 111,
            green_max: 115,
            red_min: 135,
            red_max: 139,
        },
        FuzzyPixel {
            blue_min: 30,
            blue_max: 34,
            green_min: 30,
            green_max: 34,
            red_min: 46,
            red_max: 50,
        },
        INVENTORY_BACKGROUND,
    ];

    pub const INVENTORY_RAW_ANCHOVIES: InventorySlotPixels = [
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 108,
            blue_max: 112,
            green_min: 80,
            green_max: 84,
            red_min: 79,
            red_max: 83,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 0,
            blue_max: 3,
            green_min: 0,
            green_max: 2,
            red_min: 0,
            red_max: 2,
        },
        FuzzyPixel {
            blue_min: 51,
            blue_max: 55,
            green_min: 45,
            green_max: 49,
            red_min: 45,
            red_max: 49,
        },
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 101,
            blue_max: 105,
            green_min: 74,
            green_max: 78,
            red_min: 73,
            red_max: 77,
        },
        FuzzyPixel {
            blue_min: 30,
            blue_max: 34,
            green_min: 30,
            green_max: 34,
            red_min: 46,
            red_max: 50,
        },
        INVENTORY_BACKGROUND,
    ];

    pub const INVENTORY_TINDERBOX: InventorySlotPixels = [
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 91,
            blue_max: 95,
            green_min: 91,
            green_max: 95,
            red_min: 100,
            red_max: 104,
        },
        FuzzyPixel {
            blue_min: 87,
            blue_max: 91,
            green_min: 87,
            green_max: 91,
            red_min: 96,
            red_max: 100,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 128,
            blue_max: 132,
            green_min: 129,
            green_max: 133,
            red_min: 142,
            red_max: 146,
        },
        FuzzyPixel {
            blue_min: 1,
            blue_max: 5,
            green_min: 39,
            green_max: 43,
            red_min: 67,
            red_max: 71,
        },
        FuzzyPixel {
            blue_min: 1,
            blue_max: 5,
            green_min: 39,
            green_max: 43,
            red_min: 67,
            red_max: 71,
        },
        FuzzyPixel {
            blue_min: 30,
            blue_max: 34,
            green_min: 30,
            green_max: 34,
            red_min: 46,
            red_max: 50,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 121,
            blue_max: 125,
            green_min: 121,
            green_max: 125,
            red_min: 133,
            red_max: 137,
        },
        FuzzyPixel {
            blue_min: 0,
            blue_max: 3,
            green_min: 0,
            green_max: 2,
            red_min: 0,
            red_max: 2,
        },
        INVENTORY_BACKGROUND,
    ];

    pub const INVENTORY_OAK_LOGS: InventorySlotPixels = [
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 21,
            blue_max: 25,
            green_min: 44,
            green_max: 48,
            red_min: 62,
            red_max: 66,
        },
        FuzzyPixel {
            blue_min: 35,
            blue_max: 39,
            green_min: 71,
            green_max: 75,
            red_min: 102,
            red_max: 106,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 61,
            blue_max: 65,
            green_min: 99,
            green_max: 103,
            red_min: 132,
            red_max: 136,
        },
        FuzzyPixel {
            blue_min: 28,
            blue_max: 32,
            green_min: 58,
            green_max: 62,
            red_min: 83,
            red_max: 87,
        },
        FuzzyPixel {
            blue_min: 39,
            blue_max: 43,
            green_min: 79,
            green_max: 83,
            red_min: 111,
            red_max: 115,
        },
        FuzzyPixel {
            blue_min: 0,
            blue_max: 3,
            green_min: 0,
            green_max: 2,
            red_min: 0,
            red_max: 2,
        },
        INVENTORY_BACKGROUND,
        FuzzyPixel {
            blue_min: 40,
            blue_max: 44,
            green_min: 81,
            green_max: 85,
            red_min: 114,
            red_max: 118,
        },
        INVENTORY_BACKGROUND,
        INVENTORY_BACKGROUND,
    ];
}
