/// This file holds the pixels of interest.
///
/// All values are given for the second lowest brightness setting (default for
/// starting the game.)
///
/// In case of future changes to handle other brightnesses I am wrapping these
/// constants into functions.
pub mod pixels {
    use crate::Pixel;

    pub fn red() -> Pixel {
        Pixel {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> Pixel {
        Pixel {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> Pixel {
        Pixel {
            red: 0,
            green: 0,
            blue: 255,
        }
    }
}

pub mod fuzzy_pixels {
    use crate::FuzzyPixel;

    /// Colors used in the action text at the top left of the screen.
    pub fn action_text_white() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 180,
            blue_max: 255,
            green_min: 180,
            green_max: 255,
            red_min: 180,
            red_max: 255,
        }
    }
    pub fn action_text_blue() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 180,
            blue_max: 255,
            green_min: 180,
            green_max: 255,
            red_min: 0,
            red_max: 25,
        }
    }
    pub fn action_text_yellow() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 0,
            blue_max: 30,
            green_min: 190,
            green_max: 235,
            red_min: 190,
            red_max: 235,
        }
    }

    /// Pixels for handling the banking interface.
    pub fn bank_quantity_on() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 22,
            blue_max: 35,
            green_min: 27,
            green_max: 35,
            red_min: 115,
            red_max: 138,
        }
    }
    pub fn bank_quantity_off() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 56,
            blue_max: 85,
            green_min: 61,
            green_max: 90,
            red_min: 64,
            red_max: 89,
        }
    }

    /// Pixels used to identify the bank when we are in it to select and open
    /// the banking interface.
    pub fn bank_brown1() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 1,
            blue_max: 5,
            green_min: 30,
            green_max: 40,
            red_min: 50,
            red_max: 60,
        }
    }
    pub fn bank_brown2() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 1,
            blue_max: 5,
            green_min: 40,
            green_max: 50,
            red_min: 61,
            red_max: 71,
        }
    }
    pub fn bank_brown3() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 20,
            blue_max: 24,
            green_min: 62,
            green_max: 66,
            red_min: 85,
            red_max: 89,
        }
    }

    /// Pixels used to identify things on the maps (minimap or worldmap).
    pub fn map_icon_bank_yellow() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 74,
            blue_max: 78,
            green_min: 207,
            green_max: 211,
            red_min: 230,
            red_max: 234,
        }
    }
    pub fn map_icon_fish_dark_blue() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 218,
            blue_max: 222,
            green_min: 56,
            green_max: 60,
            red_min: 5,
            red_max: 9,
        }
    }
    pub fn map_icon_fish_light_blue() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 246,
            blue_max: 255,
            green_min: 101,
            green_max: 135,
            red_min: 35,
            red_max: 43,
        }
    }
    pub fn map_icon_dark_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 157,
            blue_max: 161,
            green_min: 158,
            green_max: 162,
            red_min: 158,
            red_max: 162,
        }
    }
    pub fn map_icon_light_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 190,
            blue_max: 194,
            green_min: 189,
            green_max: 193,
            red_min: 189,
            red_max: 193,
        }
    }

    pub fn inventory_background() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 37,
            blue_max: 46,
            green_min: 49,
            green_max: 57,
            red_min: 58,
            red_max: 65,
        }
    }

    /// This is the red color that the inventory icon at the bottom of the
    /// screen turns when the inventory is open.
    pub fn inventory_icon_background_open() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 25,
            blue_max: 35,
            green_min: 35,
            green_max: 45,
            red_min: 110,
            red_max: 130,
        }
    }

    pub fn tree_bark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 40,
            blue_max: 44,
            green_min: 81,
            green_max: 85,
            red_min: 114,
            red_max: 118,
        }
    }
    pub fn oak_bark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 40,
            blue_max: 44,
            green_min: 81,
            green_max: 85,
            red_min: 114,
            red_max: 118,
        }
    }
    pub fn small_net_fishing_spot() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 105,
            blue_max: 115,
            green_min: 115,
            green_max: 135,
            red_min: 140,
            red_max: 155,
        }
    }
    // Fires seem to be dominated by 2 shades. Since they are fairly distinct
    // they should be searched seperately.
    pub fn fire_dark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 25,
            blue_max: 45,
            green_min: 125,
            green_max: 145,
            red_min: 180,
            red_max: 190,
        }
    }
    pub fn fire_light() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 25,
            blue_max: 45,
            green_min: 160,
            green_max: 190,
            red_min: 220,
            red_max: 255,
        }
    }
}

/// We check each inventory slot at a set interval. Each item in the inventory
/// has the same pixels regardless of which slot it is in. Use this to match
/// them. See location.rs for details.
///
/// When the bank is open the shadows for the items disappears so some of the
/// pixels must change.
pub mod inventory_slot_pixels {
    use super::fuzzy_pixels::inventory_background;
    use crate::Locations;
    use crate::{FuzzyPixel, InventorySlotPixels};

    pub fn empty() -> InventorySlotPixels {
        [inventory_background(); Locations::NUM_CHECKS_PER_INVENTORY_SLOT]
    }
    pub fn raw_shrimp() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 105,
                blue_max: 109,
                green_min: 119,
                green_max: 123,
                red_min: 145,
                red_max: 149,
            },
            inventory_background(),
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
            inventory_background(),
            inventory_background(),
            inventory_background(),
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
            inventory_background(),
        ]
    }
    pub fn raw_shrimp_bank() -> InventorySlotPixels {
        let mut pixels = raw_shrimp();
        pixels[10] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn raw_anchovies() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 108,
                blue_max: 112,
                green_min: 80,
                green_max: 84,
                red_min: 79,
                red_max: 83,
            },
            inventory_background(),
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
            inventory_background(),
            inventory_background(),
            inventory_background(),
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
            inventory_background(),
        ]
    }
    pub fn raw_anchovies_bank() -> InventorySlotPixels {
        let mut pixels = raw_anchovies();
        pixels[10] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn tinderbox() -> InventorySlotPixels {
        [
            inventory_background(),
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
            inventory_background(),
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
            inventory_background(),
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
            inventory_background(),
        ]
    }
    pub fn oak_logs() -> InventorySlotPixels {
        [
            inventory_background(),
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
            inventory_background(),
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
            inventory_background(),
            FuzzyPixel {
                blue_min: 40,
                blue_max: 44,
                green_min: 81,
                green_max: 85,
                red_min: 114,
                red_max: 118,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
}
