pub mod constants;
pub mod controller;
pub mod types;

pub use constants::*;
pub use controller::{InputBot, MouseMover};
pub use types::*;
pub use uinput::event::keyboard::Key;

use structopt::StructOpt;
#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(long)]
    pub mouse_paths_fpath: String, // Bincode file to read mouse positions from.
}
