use std::num::ParseIntError;
use std::str::FromStr;

/// BGR/RBG pixel. Alpha is left out since I have yet to come across an instance
/// where I care about it.
#[derive(Debug, Clone, PartialEq)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

/// Diecribes the type of pixel we expected, providing bounds on the colors it
/// will match with. Both min and max are included in matching.
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
    /// Check that 'pixel' is within the bounds set by the fuzzy pixel.
    pub fn contains(&self, pixel: &Pixel) -> bool {
        pixel.blue >= self.blue_min
            && pixel.blue <= self.blue_max
            && pixel.green >= self.green_min
            && pixel.green <= self.green_max
            && pixel.red >= self.red_min
            && pixel.red <= self.red_max
    }

    /// Check that 'pixel' is 'contain'ed within this FuzzyPixel and also checks
    /// that the ratio between the colors is acceptable.
    pub fn matches(&self, pixel: &Pixel) -> bool {
        let res = self.contains(pixel);
        // println!("{:?}, {}", pixel, res);
        res
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
