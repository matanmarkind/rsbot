/// This file holds the pixels of interest.
///
/// All values are given for the second lowest brightness setting (default for
/// starting the game.)
///
/// In case of future changes to handle other brightnesses I am wrapping these
/// constants into functions.
use crate::types::FuzzyPixel;
use crate::Locations;

pub type InventorySlotPixels = [FuzzyPixel; Locations::NUM_CHECKS_PER_INVENTORY_SLOT];

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

    pub fn black() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 0,
            blue_max: 3,
            green_min: 0,
            green_max: 3,
            red_min: 0,
            red_max: 3,
        }
    }

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
    pub fn action_text_green() -> FuzzyPixel {
        // Used when an enemy is 10+ combat levels below us.
        FuzzyPixel {
            blue_min: 0,
            blue_max: 30,
            green_min: 190,
            green_max: 235,
            red_min: 0,
            red_max: 30,
        }
    }
    pub fn action_text_orange() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 45,
            blue_max: 75,
            green_min: 106,
            green_max: 140,
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
    pub fn varrock_bank_window1() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 89,
            blue_max: 94,
            green_min: 109,
            green_max: 116,
            red_min: 117,
            red_max: 129,
        }
    }
    // Falador bank corlors are from behind the counter. This is because reset
    // causes us to see the bank from this angle.
    pub fn falador_bank_brown1() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 32,
            blue_max: 43,
            green_min: 65,
            green_max: 83,
            red_min: 93,
            red_max: 116,
        }
    }
    pub fn falador_bank_brown2() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 28,
            blue_max: 36,
            green_min: 63,
            green_max: 77,
            red_min: 83,
            red_max: 100,
        }
    }

    pub fn anvil_light_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 60,
            blue_max: 70,
            green_min: 60,
            green_max: 70,
            red_min: 70,
            red_max: 80,
        }
    }
    pub fn anvil_dark_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 25,
            blue_max: 35,
            green_min: 25,
            green_max: 35,
            red_min: 30,
            red_max: 40,
        }
    }

    pub fn cow_white() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 96,
            blue_max: 119,
            green_min: 102,
            green_max: 152,
            red_min: 107,
            red_max: 158,
        }
    }
    pub fn cow_black() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 3,
            blue_max: 33,
            green_min: 9,
            green_max: 49,
            red_min: 12,
            red_max: 56,
        }
    }
    pub fn cow_dark_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 23,
            blue_max: 52,
            green_min: 29,
            green_max: 59,
            red_min: 41,
            red_max: 71,
        }
    }
    pub fn cow_light_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 69,
            blue_max: 88,
            green_min: 90,
            green_max: 101,
            red_min: 115,
            red_max: 121,
        }
    }

    pub fn chicken_beige1() -> FuzzyPixel {
        // 113, 157, 184
        // 102, 147, 173
        FuzzyPixel {
            blue_min: 100,
            blue_max: 115,
            green_min: 145,
            green_max: 159,
            red_min: 171,
            red_max: 186,
        }
    }
    pub fn chicken_beige2() -> FuzzyPixel {
        // 86, 122, 145
        // 75, 107, 127
        FuzzyPixel {
            blue_min: 73,
            blue_max: 88,
            green_min: 124,
            green_max: 105,
            red_min: 125,
            red_max: 147,
        }
    }
    pub fn chicken_brown() -> FuzzyPixel {
        // 35, 71, 101
        // 23, 62, 94
        FuzzyPixel {
            blue_min: 21,
            blue_max: 37,
            green_min: 60,
            green_max: 73,
            red_min: 92,
            red_max: 103,
        }
    }

    pub fn al_kharid_warrior_purple1() -> FuzzyPixel {
        // 103, 5, 105
        // 81, 5, 82
        FuzzyPixel {
            blue_min: 79,
            blue_max: 105,
            green_min: 3,
            green_max: 7,
            red_min: 80,
            red_max: 107,
        }
    }
    pub fn al_kharid_warrior_purple2() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 55,
            blue_max: 30,
            green_min: 5,
            green_max: 0,
            red_min: 55,
            red_max: 30,
        }
    }

    pub fn run_icon_on() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 100,
            blue_max: 106,
            green_min: 215,
            green_max: 221,
            red_min: 233,
            red_max: 239,
        }
    }

    pub fn enemy_healthbar_red() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 15,
            blue_max: 30,
            green_min: 15,
            green_max: 30,
            red_min: 90,
            red_max: 110,
        }
    }
    pub fn enemy_healthbar_green() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 48,
            blue_max: 60,
            green_min: 132,
            green_max: 145,
            red_min: 2,
            red_max: 15,
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
    pub fn map_icon_fish_medium_blue() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 246,
            blue_max: 250,
            green_min: 101,
            green_max: 105,
            red_min: 35,
            red_max: 39,
        }
    }
    pub fn map_icon_fish_light_blue() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 251,
            blue_max: 255,
            green_min: 131,
            green_max: 135,
            red_min: 39,
            red_max: 43,
        }
    }
    pub fn map_icon_cookrange_light_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 36,
            blue_max: 40,
            green_min: 91,
            green_max: 95,
            red_min: 153,
            red_max: 157,
        }
    }
    pub fn map_icon_cookrange_medium_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 21,
            blue_max: 25,
            green_min: 58,
            green_max: 62,
            red_min: 99,
            red_max: 103,
        }
    }
    pub fn map_icon_cookrange_dark_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 10,
            blue_max: 14,
            green_min: 38,
            green_max: 42,
            red_min: 79,
            red_max: 83,
        }
    }
    pub fn map_icon_anvil_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 59,
            blue_max: 63,
            green_min: 61,
            green_max: 65,
            red_min: 61,
            red_max: 65,
        }
    }
    pub fn map_icon_pickaxe_light_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 105,
            blue_max: 120,
            green_min: 106,
            green_max: 120,
            red_min: 104,
            red_max: 120,
        }
    }
    pub fn map_icon_pickaxe_dark_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 94,
            blue_max: 98,
            green_min: 94,
            green_max: 98,
            red_min: 96,
            red_max: 100,
        }
    }
    pub fn map_icon_pickaxe_handle_light_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 61,
            blue_max: 65,
            green_min: 98,
            green_max: 102,
            red_min: 146,
            red_max: 150,
        }
    }
    pub fn map_icon_pickaxe_handle_medium_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 31,
            blue_max: 35,
            green_min: 79,
            green_max: 83,
            red_min: 119,
            red_max: 123,
        }
    }
    pub fn map_icon_pickaxe_handle_dark_brown() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 9,
            blue_max: 14,
            green_min: 38,
            green_max: 58,
            red_min: 79,
            red_max: 92,
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
    pub fn map_icon_furnace_yellow() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 8,
            blue_max: 12,
            green_min: 250,
            green_max: 255,
            red_min: 250,
            red_max: 255,
        }
    }
    pub fn map_icon_furnace_orange1() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 38,
            blue_max: 42,
            green_min: 170,
            green_max: 174,
            red_min: 237,
            red_max: 241,
        }
    }
    pub fn map_icon_furnace_orange2() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 28,
            blue_max: 32,
            green_min: 113,
            green_max: 117,
            red_min: 250,
            red_max: 255,
        }
    }
    pub fn map_icon_furnace_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 59,
            blue_max: 63,
            green_min: 61,
            green_max: 65,
            red_min: 61,
            red_max: 65,
        }
    }
    pub fn map_holiday_item_trader_icon_red() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 38,
            blue_max: 46,
            green_min: 40,
            green_max: 48,
            red_min: 190,
            red_max: 210,
        }
    }
    pub fn map_border_white() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 237,
            blue_max: 241,
            green_min: 243,
            green_max: 247,
            red_min: 233,
            red_max: 237,
        }
    }
    pub fn map_floor_beige() -> FuzzyPixel {
        // 60, 93, 101
        // 76, 124, 130
        FuzzyPixel {
            blue_min: 55,
            blue_max: 85,
            green_min: 90,
            green_max: 130,
            red_min: 95,
            red_max: 135,
        }
    }
    pub fn map_floor_gray() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 75,
            blue_max: 110,
            green_min: 79,
            green_max: 120,
            red_min: 80,
            red_max: 120,
        }
    }
    pub fn map_floor_brown() -> FuzzyPixel {
        // Minimap and worldmap are slightly different
        // Minimap seems to change colors...
        FuzzyPixel {
            blue_min: 20,
            blue_max: 55,
            green_min: 40,
            green_max: 90,
            red_min: 55,
            red_max: 110,
        }
    }
    pub fn map_varrock_west_mining_ground_brown() -> FuzzyPixel {
        // Minimap and worldmap are slightly different
        // Minimap seems to change colors...
        FuzzyPixel {
            blue_min: 10,
            blue_max: 30,
            green_min: 30,
            green_max: 60,
            red_min: 50,
            red_max: 90,
        }
    }

    // It seems that a lot of things in the map change color throughout the day,
    // so these may be unreliable.
    pub fn map_plant_green() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 68,
            blue_max: 72,
            green_min: 117,
            green_max: 121,
            red_min: 79,
            red_max: 83,
        }
    }
    pub fn map_all_trees_leaves_light() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 23,
            blue_max: 27,
            green_min: 87,
            green_max: 91,
            red_min: 28,
            red_max: 32,
        }
    }
    pub fn map_all_trees_leaves_dark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 26,
            blue_max: 30,
            green_min: 65,
            green_max: 69,
            red_min: 23,
            red_max: 27,
        }
    }
    pub fn map_oak_leaves_dark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 21,
            blue_max: 25,
            green_min: 57,
            green_max: 61,
            red_min: 55,
            red_max: 59,
        }
    }
    pub fn map_tree_bark_dark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 26,
            blue_max: 30,
            green_min: 55,
            green_max: 59,
            red_min: 80,
            red_max: 84,
        }
    }
    pub fn map_tree_bark_light() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 35,
            blue_max: 39,
            green_min: 70,
            green_max: 74,
            red_min: 91,
            red_max: 95,
        }
    }
    pub fn map_oak_bark_dark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 35,
            blue_max: 39,
            green_min: 34,
            green_max: 38,
            red_min: 31,
            red_max: 35,
        }
    }
    pub fn map_oak_bark_light() -> FuzzyPixel {
        map_tree_bark_light()
    }
    pub fn map_willow_bark() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 21,
            blue_max: 25,
            green_min: 57,
            green_max: 61,
            red_min: 55,
            red_max: 59,
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
    pub fn inventory_background_dark() -> FuzzyPixel {
        // Parts of the inventory have dark splotches.
        FuzzyPixel {
            blue_min: 30,
            blue_max: 34,
            green_min: 30,
            green_max: 34,
            red_min: 46,
            red_max: 50,
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
            red_min: 100,
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
    pub fn willow_bark1() -> FuzzyPixel {
        // 23, 46, 50
        FuzzyPixel {
            blue_min: 21,
            blue_max: 25,
            green_min: 44,
            green_max: 48,
            red_min: 48,
            red_max: 52,
        }
    }
    pub fn willow_bark2() -> FuzzyPixel {
        // 33, 63, 70
        // 34, 65, 73
        // 23, 46, 50
        // 39, 76, 86
        FuzzyPixel {
            blue_min: 31,
            blue_max: 41,
            green_min: 61,
            green_max: 78,
            red_min: 68,
            red_max: 88,
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
    pub fn furnace_grey() -> FuzzyPixel {
        // This is the color of the furnace near the player.
        FuzzyPixel {
            blue_min: 18,
            blue_max: 28,
            green_min: 18,
            green_max: 28,
            red_min: 20,
            red_max: 31,
        }
    }
    pub fn cookrange_medium_red() -> FuzzyPixel {
        // This is the color of the "fire" inside of the cook range.
        FuzzyPixel {
            blue_min: 7,
            blue_max: 12,
            green_min: 18,
            green_max: 24,
            red_min: 165,
            red_max: 183,
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
    pub fn tin_ore() -> FuzzyPixel {
        // Max BGR could be 130, 131, 144.
        // Mi BGR could be 81, 81, 80.
        // This overlaps with silver though so shrunk it down.
        FuzzyPixel {
            blue_min: 85,
            blue_max: 95,
            green_min: 85,
            green_max: 100,
            red_min: 85,
            red_max: 100,
        }
    }
    pub fn silver_ore() -> FuzzyPixel {
        // Min BGR could be 102, 103, 113.
        // This overlaps with tin though so shrunk it up.
        FuzzyPixel {
            blue_min: 112,
            blue_max: 160,
            green_min: 113,
            green_max: 160,
            red_min: 123,
            red_max: 172,
        }
    }
    pub fn copper_ore() -> FuzzyPixel {
        // Main thing to avoid is iron which is a darker brown.
        // There is a spot on my characters head which is 50, 76, 109
        // 52, 94, 147
        // 29, 53, 82
        FuzzyPixel {
            blue_min: 40,
            blue_max: 48,
            green_min: 70,
            green_max: 90,
            red_min: 100,
            red_max: 140,
        }
    }

    pub fn al_kharid_door1() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 13,
            blue_max: 17,
            green_min: 71,
            green_max: 75,
            red_min: 101,
            red_max: 105,
        }
    }
    pub fn al_kharid_door2() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 12,
            blue_max: 16,
            green_min: 60,
            green_max: 64,
            red_min: 86,
            red_max: 90,
        }
    }
    pub fn al_kharid_door3() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 7,
            blue_max: 11,
            green_min: 44,
            green_max: 48,
            red_min: 63,
            red_max: 67,
        }
    }
    pub fn al_kharid_door4() -> FuzzyPixel {
        FuzzyPixel {
            blue_min: 5,
            blue_max: 9,
            green_min: 32,
            green_max: 36,
            red_min: 46,
            red_max: 50,
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
    use super::fuzzy_pixels::{black, inventory_background, inventory_background_dark};
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
            inventory_background_dark(),
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
    pub fn cooked_shrimp() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 7,
                blue_max: 11,
                green_min: 90,
                green_max: 94,
                red_min: 151,
                red_max: 155,
            },
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 1,
                blue_max: 5,
                green_min: 31,
                green_max: 35,
                red_min: 76,
                red_max: 80,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 7,
                blue_max: 11,
                green_min: 83,
                green_max: 87,
                red_min: 141,
                red_max: 145,
            },
            inventory_background_dark(),
            inventory_background(),
        ]
    }
    pub fn cooked_shrimp_bank() -> InventorySlotPixels {
        let mut pixels = cooked_shrimp();
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
    pub fn burned_shrimp() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 57,
                blue_max: 61,
                green_min: 57,
                green_max: 61,
                red_min: 62,
                red_max: 66,
            },
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 18,
                blue_max: 22,
                green_min: 18,
                green_max: 22,
                red_min: 20,
                red_max: 24,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 52,
                blue_max: 56,
                green_min: 52,
                green_max: 56,
                red_min: 58,
                red_max: 62,
            },
            inventory_background_dark(),
            inventory_background(),
        ]
    }
    pub fn burned_shrimp_bank() -> InventorySlotPixels {
        let mut pixels = burned_shrimp();
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
            inventory_background_dark(),
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
    pub fn cooked_anchovies() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 95,
                blue_max: 99,
                green_min: 50,
                green_max: 54,
                red_min: 44,
                red_max: 48,
            },
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 54,
                blue_max: 58,
                green_min: 33,
                green_max: 37,
                red_min: 31,
                red_max: 35,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 92,
                blue_max: 96,
                green_min: 48,
                green_max: 52,
                red_min: 42,
                red_max: 46,
            },
            inventory_background_dark(),
            inventory_background(),
        ]
    }
    pub fn cooked_anchovies_bank() -> InventorySlotPixels {
        let mut pixels = cooked_anchovies();
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
            inventory_background_dark(),
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
    pub fn tree_logs() -> InventorySlotPixels {
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
                blue_min: 29,
                blue_max: 33,
                green_min: 60,
                green_max: 64,
                red_min: 86,
                red_max: 90,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 50,
                blue_max: 54,
                green_min: 107,
                green_max: 111,
                red_min: 142,
                red_max: 146,
            },
            FuzzyPixel {
                blue_min: 24,
                blue_max: 38,
                green_min: 51,
                green_max: 55,
                red_min: 73,
                red_max: 77,
            },
            FuzzyPixel {
                blue_min: 32,
                blue_max: 36,
                green_min: 67,
                green_max: 71,
                red_min: 96,
                red_max: 100,
            },
black(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 33,
                blue_max: 37,
                green_min: 69,
                green_max: 73,
                red_min: 99,
                red_max: 103,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn tree_logs_bank() -> InventorySlotPixels {
        tree_logs()
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
    pub fn oak_logs_bank() -> InventorySlotPixels {
        oak_logs()
    }
    pub fn willow_logs() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 25,
                green_max: 29,
                red_min: 31,
                red_max: 35,
            },
            FuzzyPixel {
                blue_min: 10,
                blue_max: 14,
                green_min: 44,
                green_max: 48,
                red_min: 52,
                red_max: 56,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 35,
                blue_max: 39,
                green_min: 69,
                green_max: 73,
                red_min: 78,
                red_max: 82,
            },
            FuzzyPixel {
                blue_min: 8,
                blue_max: 12,
                green_min: 33,
                green_max: 37,
                red_min: 40,
                red_max: 44,
            },
            FuzzyPixel {
                blue_min: 12,
                blue_max: 16,
                green_min: 47,
                green_max: 51,
                red_min: 55,
                red_max: 59,
            },
            black(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 13,
                blue_max: 17,
                green_min: 51,
                green_max: 55,
                red_min: 59,
                red_max: 63,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn willow_logs_bank() -> InventorySlotPixels {
        willow_logs()
    }
    pub fn tin_ore() -> InventorySlotPixels {
        [
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 93,
                blue_max: 97,
                green_min: 93,
                green_max: 97,
                red_min: 103,
                red_max: 107,
            },
            black(),
            FuzzyPixel {
                blue_min: 24,
                blue_max: 28,
                green_min: 41,
                green_max: 45,
                red_min: 55,
                red_max: 59,
            },
            inventory_background_dark(),
            FuzzyPixel {
                blue_min: 87,
                blue_max: 91,
                green_min: 87,
                green_max: 91,
                red_min: 96,
                red_max: 100,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 37,
                blue_max: 41,
                green_min: 62,
                green_max: 66,
                red_min: 84,
                red_max: 88,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn tin_ore_bank() -> InventorySlotPixels {
        let mut pixels = tin_ore();
        pixels[5] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn iron_ore() -> InventorySlotPixels {
        [
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 15,
                blue_max: 19,
                green_min: 24,
                green_max: 28,
                red_min: 48,
                red_max: 52,
            },
            black(),
            FuzzyPixel {
                blue_min: 24,
                blue_max: 28,
                green_min: 41,
                green_max: 45,
                red_min: 55,
                red_max: 59,
            },
            inventory_background_dark(),
            FuzzyPixel {
                blue_min: 12,
                blue_max: 16,
                green_min: 20,
                green_max: 24,
                red_min: 41,
                red_max: 45,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 37,
                blue_max: 41,
                green_min: 62,
                green_max: 66,
                red_min: 84,
                red_max: 88,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn iron_ore_bank() -> InventorySlotPixels {
        let mut pixels = iron_ore();
        pixels[5] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn silver_ore() -> InventorySlotPixels {
        [
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 147,
                blue_max: 151,
                green_min: 133,
                green_max: 137,
                red_min: 133,
                red_max: 137,
            },
            black(),
            FuzzyPixel {
                blue_min: 24,
                blue_max: 28,
                green_min: 41,
                green_max: 45,
                red_min: 55,
                red_max: 59,
            },
            inventory_background_dark(),
            FuzzyPixel {
                blue_min: 138,
                blue_max: 142,
                green_min: 125,
                green_max: 129,
                red_min: 124,
                red_max: 128,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 37,
                blue_max: 41,
                green_min: 62,
                green_max: 66,
                red_min: 84,
                red_max: 88,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn copper_ore() -> InventorySlotPixels {
        [
            inventory_background(),
            black(),
            FuzzyPixel {
                blue_min: 28,
                blue_max: 32,
                green_min: 90,
                green_max: 94,
                red_min: 191,
                red_max: 195,
            },
            black(),
            FuzzyPixel {
                blue_min: 24,
                blue_max: 28,
                green_min: 41,
                green_max: 45,
                red_min: 55,
                red_max: 59,
            },
            inventory_background_dark(),
            FuzzyPixel {
                blue_min: 27,
                blue_max: 31,
                green_min: 84,
                green_max: 88,
                red_min: 179,
                red_max: 183,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 37,
                blue_max: 41,
                green_min: 62,
                green_max: 66,
                red_min: 84,
                red_max: 88,
            },
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn copper_ore_bank() -> InventorySlotPixels {
        let mut pixels = copper_ore();
        pixels[5] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn silver_ore_bank() -> InventorySlotPixels {
        let mut pixels = silver_ore();
        pixels[5] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }
    pub fn uncut_sapphire() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 91,
                blue_max: 95,
                green_min: 4,
                green_max: 8,
                red_min: 3,
                red_max: 7,
            },
            black(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 105,
                blue_max: 109,
                green_min: 5,
                green_max: 9,
                red_min: 4,
                red_max: 8,
            },
            FuzzyPixel {
                blue_min: 73,
                blue_max: 77,
                green_min: 3,
                green_max: 7,
                red_min: 1,
                red_max: 5,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 99,
                blue_max: 103,
                green_min: 5,
                green_max: 9,
                red_min: 3,
                red_max: 7,
            },
            FuzzyPixel {
                blue_min: 70,
                blue_max: 74,
                green_min: 3,
                green_max: 7,
                red_min: 1,
                red_max: 5,
            },
            inventory_background(),
        ]
    }
    pub fn uncut_sapphire_bank() -> InventorySlotPixels {
        uncut_sapphire()
    }
    pub fn uncut_ruby() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 1,
                blue_max: 5,
                green_min: 7,
                green_max: 11,
                red_min: 70,
                red_max: 74,
            },
            black(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 3,
                blue_max: 7,
                green_min: 8,
                green_max: 12,
                red_min: 82,
                red_max: 86,
            },
            FuzzyPixel {
                blue_min: 0,
                blue_max: 4,
                green_min: 4,
                green_max: 8,
                red_min: 56,
                red_max: 60,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 1,
                blue_max: 5,
                green_min: 7,
                green_max: 11,
                red_min: 76,
                red_max: 80,
            },
            FuzzyPixel {
                blue_min: 0,
                blue_max: 4,
                green_min: 4,
                green_max: 8,
                red_min: 56,
                red_max: 60,
            },
            inventory_background(),
        ]
    }
    pub fn uncut_ruby_bank() -> InventorySlotPixels {
        uncut_ruby()
    }
    pub fn clay() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 58,
                blue_max: 62,
                green_min: 101,
                green_max: 105,
                red_min: 127,
                red_max: 131,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn clay_bank() -> InventorySlotPixels {
        clay()
    }
    pub fn bronze_bar() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 15,
                blue_max: 19,
                green_min: 33,
                green_max: 37,
                red_min: 48,
                red_max: 52,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 7,
                blue_max: 11,
                green_min: 17,
                green_max: 21,
                red_min: 24,
                red_max: 28,
            },
            FuzzyPixel {
                blue_min: 21,
                blue_max: 25,
                green_min: 44,
                green_max: 48,
                red_min: 62,
                red_max: 66,
            },
            inventory_background_dark(),
            inventory_background(),
            inventory_background_dark(),
            inventory_background_dark(),
            inventory_background(),
        ]
    }
    pub fn bronze_bar_bank() -> InventorySlotPixels {
        let mut pixels = bronze_bar();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels[9] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
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

    pub fn bronze_platelegs() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 0,
                blue_max: 2,
                green_min: 20,
                green_max: 24,
                red_min: 29,
                red_max: 33,
            },
            inventory_background_dark(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 23,
                blue_max: 27,
                green_min: 46,
                green_max: 50,
                red_min: 67,
                red_max: 71,
            },
            inventory_background_dark(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 0,
                blue_max: 2,
                green_min: 25,
                green_max: 29,
                red_min: 40,
                red_max: 44,
            },
            inventory_background_dark(),
            inventory_background(),
        ]
    }
    pub fn bronze_platelegs_bank() -> InventorySlotPixels {
        let mut pixels = bronze_platelegs();
        pixels[2] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels[6] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
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

    pub fn pizza_base() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 76,
                blue_max: 80,
                green_min: 103,
                green_max: 107,
                red_min: 103,
                red_max: 107,
            },
            FuzzyPixel {
                blue_min: 40,
                blue_max: 44,
                green_min: 55,
                green_max: 59,
                red_min: 55,
                red_max: 59,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 118,
                blue_max: 122,
                green_min: 161,
                green_max: 165,
                red_min: 162,
                red_max: 166,
            },
            FuzzyPixel {
                blue_min: 92,
                blue_max: 96,
                green_min: 126,
                green_max: 130,
                red_min: 127,
                red_max: 131,
            },
            FuzzyPixel {
                blue_min: 63,
                blue_max: 67,
                green_min: 86,
                green_max: 90,
                red_min: 86,
                red_max: 90,
            },
            inventory_background_dark(),
            inventory_background(),
            black(),
            black(),
            inventory_background(),
        ]
    }
    pub fn pizza_base_bank() -> InventorySlotPixels {
        let mut pixels = pizza_base();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn tomato() -> InventorySlotPixels {
        [
            inventory_background(),
            black(),
            inventory_background_dark(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 15,
                green_max: 19,
                red_min: 133,
                red_max: 137,
            },
            FuzzyPixel {
                blue_min: 3,
                blue_max: 7,
                green_min: 10,
                green_max: 14,
                red_min: 103,
                red_max: 107,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn tomato_bank() -> InventorySlotPixels {
        let mut pixels = tomato();
        pixels[2] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn cheese() -> InventorySlotPixels {
        [
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 8,
                blue_max: 12,
                green_min: 144,
                green_max: 148,
                red_min: 172,
                red_max: 176,
            },
            FuzzyPixel {
                blue_min: 7,
                blue_max: 11,
                green_min: 120,
                green_max: 124,
                red_min: 144,
                red_max: 148,
            },
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
            inventory_background(),
        ]
    }
    pub fn cheese_bank() -> InventorySlotPixels {
        let mut pixels = cheese();
        pixels[2] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn incomplete_pizza() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 97,
                blue_max: 101,
                green_min: 120,
                green_max: 124,
                red_min: 133,
                red_max: 137,
            },
            FuzzyPixel {
                blue_min: 54,
                blue_max: 58,
                green_min: 66,
                green_max: 70,
                red_min: 74,
                red_max: 78,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 170,
                blue_max: 174,
                green_min: 186,
                green_max: 190,
                red_min: 197,
                red_max: 201,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            inventory_background_dark(),
            inventory_background(),
            black(),
            black(),
            inventory_background(),
        ]
    }
    pub fn incomplete_pizza_bank() -> InventorySlotPixels {
        let mut pixels = incomplete_pizza();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn uncooked_pizza() -> InventorySlotPixels {
        // It's identical to uncooked_pizza...
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 97,
                blue_max: 101,
                green_min: 120,
                green_max: 124,
                red_min: 133,
                red_max: 137,
            },
            FuzzyPixel {
                blue_min: 54,
                blue_max: 58,
                green_min: 66,
                green_max: 70,
                red_min: 74,
                red_max: 78,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 170,
                blue_max: 174,
                green_min: 186,
                green_max: 190,
                red_min: 197,
                red_max: 201,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            inventory_background_dark(),
            inventory_background(),
            black(),
            black(),
            inventory_background(),
        ]
    }
    pub fn uncooked_pizza_bank() -> InventorySlotPixels {
        let mut pixels = uncooked_pizza();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn plain_pizza() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 60,
                blue_max: 64,
                green_min: 90,
                green_max: 94,
                red_min: 102,
                red_max: 106,
            },
            FuzzyPixel {
                blue_min: 31,
                blue_max: 35,
                green_min: 47,
                green_max: 51,
                red_min: 54,
                red_max: 58,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 93,
                blue_max: 97,
                green_min: 140,
                green_max: 144,
                red_min: 159,
                red_max: 163,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            inventory_background_dark(),
            inventory_background(),
            black(),
            black(),
            inventory_background(),
        ]
    }
    pub fn plain_pizza_bank() -> InventorySlotPixels {
        let mut pixels = plain_pizza();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn burnt_pizza() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 36,
                blue_max: 40,
                green_min: 36,
                green_max: 40,
                red_min: 40,
                red_max: 44,
            },
            FuzzyPixel {
                blue_min: 18,
                blue_max: 22,
                green_min: 18,
                green_max: 22,
                red_min: 20,
                red_max: 24,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 59,
                blue_max: 63,
                green_min: 59,
                green_max: 63,
                red_min: 65,
                red_max: 69,
            },
            FuzzyPixel {
                blue_min: 1,
                blue_max: 7,
                green_min: 3,
                green_max: 7,
                red_min: 48,
                red_max: 52,
            },
            FuzzyPixel {
                blue_min: 1,
                blue_max: 7,
                green_min: 3,
                green_max: 7,
                red_min: 48,
                red_max: 52,
            },
            inventory_background_dark(),
            inventory_background(),
            black(),
            black(),
            inventory_background(),
        ]
    }
    pub fn burnt_pizza_bank() -> InventorySlotPixels {
        let mut pixels = burnt_pizza();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn anchovy_pizza() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 132,
                blue_max: 136,
                green_min: 132,
                green_max: 136,
                red_min: 145,
                red_max: 149,
            },
            FuzzyPixel {
                blue_min: 29,
                blue_max: 33,
                green_min: 45,
                green_max: 49,
                red_min: 51,
                red_max: 55,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 93,
                blue_max: 97,
                green_min: 140,
                green_max: 144,
                red_min: 159,
                red_max: 163,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            FuzzyPixel {
                blue_min: 5,
                blue_max: 9,
                green_min: 12,
                green_max: 16,
                red_min: 111,
                red_max: 115,
            },
            inventory_background_dark(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 82,
                blue_max: 86,
                green_min: 123,
                green_max: 127,
                red_min: 140,
                red_max: 144,
            },
            FuzzyPixel {
                blue_min: 35,
                blue_max: 39,
                green_min: 53,
                green_max: 57,
                red_min: 60,
                red_max: 64,
            },
            inventory_background(),
        ]
    }
    pub fn anchovy_pizza_bank() -> InventorySlotPixels {
        let mut pixels = anchovy_pizza();
        pixels[7] = FuzzyPixel {
            blue_min: 49,
            blue_max: 53,
            green_min: 49,
            green_max: 53,
            red_min: 49,
            red_max: 53,
        };
        pixels
    }

    pub fn jug() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 67,
                blue_max: 71,
                green_min: 67,
                green_max: 71,
                red_min: 74,
                red_max: 78,
            },
            black(),
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 130,
                blue_max: 134,
                green_min: 131,
                green_max: 135,
                red_min: 144,
                red_max: 148,
            },
            FuzzyPixel {
                blue_min: 78,
                blue_max: 82,
                green_min: 78,
                green_max: 82,
                red_min: 86,
                red_max: 90,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 115,
                blue_max: 119,
                green_min: 116,
                green_max: 120,
                red_min: 128,
                red_max: 132,
            },
            black(),
            inventory_background(),
        ]
    }
    pub fn jug_bank() -> InventorySlotPixels {
        jug()
    }
    pub fn jug_of_water() -> InventorySlotPixels {
        jug()
    }
    pub fn jug_of_water_bank() -> InventorySlotPixels {
        jug_of_water()
    }

    pub fn pot() -> InventorySlotPixels {
        [
            inventory_background(),
            FuzzyPixel {
                blue_min: 12,
                blue_max: 16,
                green_min: 32,
                green_max: 36,
                red_min: 55,
                red_max: 59,
            },
            FuzzyPixel {
                blue_min: 20,
                blue_max: 24,
                green_min: 50,
                green_max: 54,
                red_min: 85,
                red_max: 89,
            },
            inventory_background(),
            FuzzyPixel {
                blue_min: 37,
                blue_max: 41,
                green_min: 91,
                green_max: 95,
                red_min: 154,
                red_max: 158,
            },
            FuzzyPixel {
                blue_min: 35,
                blue_max: 39,
                green_min: 84,
                green_max: 88,
                red_min: 141,
                red_max: 145,
            },
            FuzzyPixel {
                blue_min: 21,
                blue_max: 25,
                green_min: 52,
                green_max: 56,
                red_min: 88,
                red_max: 92,
            },
            inventory_background(),
            inventory_background(),
            FuzzyPixel {
                blue_min: 31,
                blue_max: 35,
                green_min: 76,
                green_max: 80,
                red_min: 129,
                red_max: 133,
            },
            FuzzyPixel {
                blue_min: 17,
                blue_max: 21,
                green_min: 41,
                green_max: 45,
                red_min: 71,
                red_max: 75,
            },
            inventory_background(),
        ]
    }
    pub fn pot_bank() -> InventorySlotPixels {
        pot()
    }
    pub fn pot_of_flour() -> InventorySlotPixels {
        let mut pixels = pot();
        pixels[1] = FuzzyPixel {
            blue_min: 223,
            blue_max: 227,
            green_min: 224,
            green_max: 228,
            red_min: 228,
            red_max: 232,
        };
        pixels[2] = FuzzyPixel {
            blue_min: 24,
            blue_max: 28,
            green_min: 60,
            green_max: 64,
            red_min: 102,
            red_max: 106,
        };
        pixels
    }
    pub fn pot_of_flour_bank() -> InventorySlotPixels {
        pot_of_flour()
    }
}
