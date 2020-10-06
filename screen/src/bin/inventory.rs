// Practice inventory frame interaction
// - Determine when open
// - Count empty slots
// - draw lines to box each slot.
// Figure out the edge padding on the inventory.

// Top left of inventory
// Dimension of each slot.A

use screen::{locations, Capturer, Frame, FuzzyPixel, Pixel};
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a slash (e.g. /path/to/dir/ on linux)"
    )]
    pub out_dir: String,
}

pub const INVENTORY_BACKGROUND: FuzzyPixel = FuzzyPixel {
    blue_min: 61,
    blue_max: 63,
    green_min: 52,
    green_max: 54,
    red_min: 40,
    red_max: 42,
};

pub const NUM_INVENTORY_ROWS: i32 = 7;
pub const NUM_INVENTORY_COLS: i32 = 4;

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    // Capture the frame and convert it to RGB for saving.
    let mut capturer = Capturer::new();

    println!("Capturing and manipulating frame.");
    let mut frame = capturer.frame().unwrap().to_owned().flip();
    for r in 0..NUM_INVENTORY_ROWS {
        for c in 0..NUM_INVENTORY_COLS {
            let DeltaPosition { dx, dy } = locations::INVENTORY_SLOT_DIMENSIONS;
            let top_left = Position {
                x: locations::INVENTORY_FIRST_SLOT.x + dx * c,
                y: locations::INVENTORY_FIRST_SLOT.y + dy * r,
            };
            frame.draw_box(
                &top_left,
                &locations::INVENTORY_SLOT_DIMENSIONS,
                &Pixel {
                    blue: 0,
                    green: 0,
                    red: 255,
                },
            );
        }
    }

    println!("Saving");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_inventory.png");
    frame.save(ofpath.as_str());
}
