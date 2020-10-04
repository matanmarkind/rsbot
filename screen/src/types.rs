use std::cmp::max;
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
        self.contains(pixel)
            && self.within_bg_ratio(pixel)
            && self.within_br_ratio(pixel)
            && self.within_gr_ratio(pixel)
    }

    fn ratio(n: u8, d: u8) -> f32 {
        let n = max(1, n);
        let d = max(1, d);
        n as f32 / d as f32
    }

    fn within_bg_ratio(&self, pixel: &Pixel) -> bool {
        let baseline = Self::ratio(self.blue_min, self.green_min)
            .max(Self::ratio(self.blue_min, self.green_max));
        let actual = Self::ratio(pixel.blue, pixel.green);
        (1.0 - CHANNEL_RATIO_SLACK) * baseline <= actual
            && (1.0 + CHANNEL_RATIO_SLACK) * baseline >= actual
    }

    fn within_br_ratio(&self, pixel: &Pixel) -> bool {
        let baseline =
            Self::ratio(self.blue_min, self.red_min).max(Self::ratio(self.blue_min, self.red_max));
        let actual = Self::ratio(pixel.blue, pixel.red);
        (1.0 - CHANNEL_RATIO_SLACK) * baseline <= actual
            && (1.0 + CHANNEL_RATIO_SLACK) * baseline >= actual
    }

    fn within_gr_ratio(&self, pixel: &Pixel) -> bool {
        let baseline = Self::ratio(self.green_min, self.red_min)
            .max(Self::ratio(self.green_min, self.red_max));
        let actual = Self::ratio(pixel.green, pixel.red);
        (1.0 - CHANNEL_RATIO_SLACK) * baseline <= actual
            && (1.0 + CHANNEL_RATIO_SLACK) * baseline >= actual
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
