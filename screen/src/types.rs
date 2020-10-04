use std::num::ParseIntError;
use std::str::FromStr;
use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action.
///
/// These are all expected to be constants, so the lifetimes will be static.
pub struct ActionLetter<'a> {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Letters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: &'a [DeltaPosition],
}

/// BGR/RBG pixel. Alpha is left out since I have yet to come across an instance
/// where I care about it.
#[derive(Debug, Clone, PartialEq)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

pub type PixelMatcher = fn(&Pixel) -> bool;

impl Pixel {
    pub fn is_white(pixel: &Pixel) -> bool {
        use std::cmp::{max, min};
        let max = max(pixel.blue, max(pixel.green, pixel.red));
        let min = min(pixel.blue, min(pixel.green, pixel.red));

        // It seems that the whites have some significant variation in their pixels.
        (max - min) <= 10 && pixel.blue > 150 && pixel.green > 150 && pixel.red > 150
    }

    /// Blue color used to show objects in the top left corner when displaying the
    /// action.
    pub fn is_letter_blue(pixel: &Pixel) -> bool {
        use std::cmp::{max, min};
        // dbg!(bgr);

        max(pixel.blue, pixel.green) - min(pixel.blue, pixel.green) <= 10
            && pixel.blue > 200
            && pixel.green > 200
            && pixel.red <= 15
    }
}

//// Type FuzzyPixe w/FromStr
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzyPixel {
    pub blue_min: u8,
    pub blue_max: u8,
    pub green_min: u8,
    pub green_max: u8,
    pub red_min: u8,
    pub red_max: u8,
}

impl FuzzyPixel {
    pub fn contains(&self, pixel: &Pixel) -> bool {
        pixel.blue >= self.blue_min
            && pixel.blue <= self.blue_max
            && pixel.green >= self.green_min
            && pixel.green <= self.green_max
            && pixel.red >= self.red_min
            && pixel.red <= self.red_max
    }
}

impl FromStr for FuzzyPixel {
    type Err = ParseIntError;

    /// Input is expected to be "1,2,3,4,5" without anything around (e.g. no
    /// "(1,2,3,4,5)")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().split(",").collect();
        Ok(FuzzyPixel {
            blue_min: coords[0].parse::<u8>()?,
            blue_max: coords[1].parse::<u8>()?,
            green_min: coords[2].parse::<u8>()?,
            green_max: coords[3].parse::<u8>()?,
            red_min: coords[4].parse::<u8>()?,
            red_max: coords[5].parse::<u8>()?,
        })
    }
}
