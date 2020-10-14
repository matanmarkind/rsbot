use crate::types::*;
use crate::{action_letters, colors, Frame, Locations};
use util::*;

/// Public interface for getting info about the screen.
///
/// Locations gives screen handling a state so we wrap uses for it. We don't
/// wrap Capturere or Frame since those create lots of complexity with lifetime
/// and borrowing that we don't want mixing with this class.
pub struct FrameHandler {
    pub locations: crate::Locations,
}

impl FrameHandler {
    pub fn new(config: crate::Config) -> FrameHandler {
        FrameHandler {
            locations: Locations::new(
                config.screen_top_left,
                util::DeltaPosition {
                    dx: config.screen_bottom_right.x - config.screen_top_left.x + 1,
                    dy: config.screen_bottom_right.y - config.screen_top_left.y + 1,
                },
            ),
        }
    }

    pub fn check_action_letters(
        &self,
        frame: &impl Frame,
        letter_and_pixels: &[(action_letters::Letter, FuzzyPixel)],
    ) -> bool {
        action_letters::check_action_letters(
            frame,
            letter_and_pixels,
            self.locations.action_text_top_left(),
        )
    }
    pub fn mark_letters_and_save(
        &self,
        frame: &impl Frame,
        fpath: &str,
        letter_and_pixels: &[(action_letters::Letter, crate::FuzzyPixel)],
    ) -> std::thread::JoinHandle<()> {
        action_letters::mark_letters_and_save(
            frame,
            fpath,
            &letter_and_pixels,
            self.locations.action_text_top_left(),
        )
    }

    pub fn is_inventory_open(&self, frame: &impl Frame) -> bool {
        // Use check_loose_pixel because the background color of the icons is very
        // distinct between on and off and the satchel depicted is also a
        // significantly different color. If the image shifts, which it sometimes
        // does I don't want to be too brittle since I think the risk of a false
        // positive is relatively low.
        frame.check_loose_pixel(
            &self.locations.inventory_icon_background(),
            &colors::INVENTORY_ICON_BACKGROUND_OPEN,
        )
    }

    pub fn is_inventory_slot_open(&self, frame: &impl Frame, slot_index: i32) -> bool {
        let top_left = self.locations.inventory_slot_top_left(slot_index);
        let dimensions = self.locations.inventory_slot_dimensions();

        let past_bottom_right = &top_left + &dimensions;
        let check_spacing = Locations::INVENTORY_SLOT_CHECK_SPACING;

        // Don't bother checking the border between slots.
        let first_pos = &top_left + &check_spacing;
        let mut pos = first_pos;
        while pos.y < past_bottom_right.y {
            while pos.x < past_bottom_right.x {
                let pixel = frame.get_pixel(&pos);
                if !colors::INVENTORY_BACKGROUND.matches(&pixel) {
                    // println!("is_slot_open={}, {:?}, {:?}", slot_index, pos, pixel);
                    return false;
                }
                pos = Position {
                    x: pos.x + check_spacing.dx,
                    y: pos.y,
                };
            }
            pos = Position {
                x: first_pos.x,
                y: pos.y + check_spacing.dy,
            };
        }
        true
    }

    /// Get the minimum slot_index [0,NUM_INVENTORY_SLOTS) which points to an open
    /// slot. Returns None if there is no open slot.
    pub fn first_open_inventory_slot(&self, frame: &impl Frame) -> Option<i32> {
        for i in 0..Locations::NUM_INVENTORY_SLOTS {
            if self.is_inventory_slot_open(frame, i) {
                return Some(i);
            }
        }
        None
    }
}
