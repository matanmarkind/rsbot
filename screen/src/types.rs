use std::cmp::{max, min};
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

// When chacking if a pixel matches a FuzzyPixel, we also want to restrict by
// the ratio between the channels, to allow wider absolute limits. The slack
// between the ratio of the inputs to the actual pixel.
const CHANNEL_RATIO_SLACK: f32 = 0.1;

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
        // dbg!(
        //     &pixel,
        //     self.within_bg_ratio(pixel),
        //     self.within_br_ratio(pixel),
        //     self.within_gr_ratio(pixel)
        // );
        let res = self.contains(pixel)
            && self.within_bg_ratio(pixel)
            && self.within_br_ratio(pixel)
            && self.within_gr_ratio(pixel);
        println!("{:?}, {}", pixel, res);
        self.contains(pixel)
    }

    fn ratio(n: u8, d: u8) -> f32 {
        let n = max(1, n);
        let d = max(1, d);
        n as f32 / d as f32
    }

    // check the ratio of 2 colors, a and b, is acceptable. min and max are the
    // boundaries set by this FuzzyPixel, and a and b are the actual values that
    // are being matches against.
    fn check_ratio(a_min: u8, a_max: u8, b_min: u8, b_max: u8, a: u8, b: u8) -> bool {
        // let ratios = [
        //     Self::ratio(a_min, b_min),
        //     Self::ratio(a_min, b_max),
        //     Self::ratio(a_max, b_min),
        //     Self::ratio(a_max, b_max),
        // ];

        // let minratio = ratios.iter().fold(f32::INFINITY, |x, &y| x.min(y));
        // let maxratio = ratios.iter().fold(f32::INFINITY, |x, &y| x.max(y));

        let minratio = max(a_min, b_min) - min(a_min, b_min);
        let maxratio = max(a_max, b_max) - min(a_max, b_max);
        let minratio = minratio.min(maxratio);
        let maxratio = minratio.max(maxratio);

        let actual = max(a, b) - min(a, b);
        (1.0 - CHANNEL_RATIO_SLACK) * minratio as f32 <= actual as f32
            && (1.0 + CHANNEL_RATIO_SLACK) * maxratio as f32 >= actual as f32
    }

    fn within_bg_ratio(&self, pixel: &Pixel) -> bool {
        Self::check_ratio(
            self.blue_min,
            self.blue_max,
            self.green_min,
            self.green_max,
            pixel.blue,
            pixel.green,
        )
    }

    fn within_br_ratio(&self, pixel: &Pixel) -> bool {
        Self::check_ratio(
            self.blue_min,
            self.blue_max,
            self.red_min,
            self.red_max,
            pixel.blue,
            pixel.red,
        )
    }

    fn within_gr_ratio(&self, pixel: &Pixel) -> bool {
        Self::check_ratio(
            self.green_min,
            self.green_max,
            self.red_min,
            self.red_max,
            pixel.green,
            pixel.red,
        )
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
