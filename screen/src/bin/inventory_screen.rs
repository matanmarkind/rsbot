// Practice inventory frame interaction
// - Determine when open
// - Count empty slots
// - draw lines to box each slot.
// Figure out the edge padding on the inventory.

// Top left of inventory
// Dimension of each slot.A

use screen::{colors, inventory, locations, Capturer, Frame};
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

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    // Capture the frame and convert it to RGB for saving.
    let mut capturer = Capturer::new();
    println!("Capturing, analyzing, and manipulating frame.");
    let mut frame = capturer.frame().unwrap().to_owned();
    frame.flip();
    for i in 0..inventory::NUM_INVENTORY_SLOTS {
        println!(
            "Is slot {} open? {}",
            i,
            inventory::is_slot_open(&mut frame, i)
        );
    }

    for r in 0..inventory::NUM_INVENTORY_ROWS {
        for c in 0..inventory::NUM_INVENTORY_COLS {
            let DeltaPosition { dx, dy } = locations::INVENTORY_SLOT_DIMENSIONS;
            let top_left = Position {
                x: locations::INVENTORY_FIRST_SLOT.x + dx * c,
                y: locations::INVENTORY_FIRST_SLOT.y + dy * r,
            };
            frame.draw_box(
                &top_left,
                &locations::INVENTORY_SLOT_DIMENSIONS,
                &colors::PURE_RED,
            );
        }
    }

    println!("Saving");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_inventory.png");
    frame.save(ofpath.as_str());
}
