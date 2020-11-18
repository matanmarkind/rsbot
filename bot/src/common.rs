use screen::{fuzzy_pixels, FuzzyPixel};
use strum_macros::EnumString;

#[derive(Debug, Copy, Clone, EnumString)]
pub enum BankLocation {
    AlKharid,
    Falador,
    VarrockWest,
    Draynor,
}

pub fn bank_pixels(loc: BankLocation) -> Vec<FuzzyPixel> {
    match loc {
        BankLocation::AlKharid | BankLocation::Draynor => vec![
            fuzzy_pixels::bank_brown1(),
            fuzzy_pixels::bank_brown2(),
            fuzzy_pixels::bank_brown3(),
        ],
        BankLocation::Falador => vec![
            fuzzy_pixels::falador_bank_brown1(),
            fuzzy_pixels::falador_bank_brown2(),
        ],
        BankLocation::VarrockWest => vec![fuzzy_pixels::varrock_bank_window1()],
    }
}
