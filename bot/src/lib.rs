pub mod controller;

pub use controller::Player;

use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub userinput_config: userinput::Config,

    #[structopt(flatten)]
    pub screen_config: screen::Config,
}
