pub mod actions;

pub use actions::*;

use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub userinput_config: userinput::Config,

    #[structopt(flatten)]
    pub screen_config: screen::Config,

    #[structopt(
        long,
        about = "Angle to pan the camera to the left if we fail to find a \
                 matching pixel/action in the open screen.",
        default_value = "37.0"
    )]
    pub bot_pan_angle_on_failure_to_find: f32,
}
