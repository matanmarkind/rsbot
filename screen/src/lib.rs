pub mod action_letters;
pub mod capture;
pub mod constants;
pub mod frame;
pub mod inventory;
pub mod locations;
pub mod types;

/// Publish internals so users can use screen::Frame instead of
/// screen::frame::Frame.
pub use capture::*;
pub use constants::*;
pub use frame::*;
pub use locations::Locations;
pub use types::*;

use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(
        long,
        about = "Top left position in pixels of the screen 'x,y'. This is the \
                 first pixel that changes color based on what is shown, not \
                 the top left pixel of the window."
    )]
    pub screen_top_left: util::Position,

    #[structopt(
        long,
        about = "Bottom right position in pixels of the screen 'x,y'. This is \
                 the last pixel that changes color based on what is shown, \
                 not the bottom right pixel of the window."
    )]
    pub screen_bottom_right: util::Position,
}

/// Public interface for getting info about the screen.
///
/// Locations gives screen handling a state so we wrap uses for it. We don't
/// wrap Capturere or Frame since those create lots of complexity with lifetime
/// and borrowing that we don't want mixing with this class.
pub struct Handler {
    pub locations: crate::Locations,
}

impl Handler {
    pub fn new(config: Config) -> Handler {
        Handler {
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
}
