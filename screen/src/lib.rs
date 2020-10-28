pub mod action_text;
pub mod colors;
pub mod frame;
pub mod locations;
pub mod types;

pub use action_text::Text as ActionText;
pub use colors::*;
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
