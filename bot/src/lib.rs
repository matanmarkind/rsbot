pub mod actions;
pub mod common;

pub use actions::*;
pub use common::*;

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

    #[structopt(
        long,
        about = "How long to run the bot for, in hours, must be under 6h.",
        default_value = "5.5"
    )]
    pub bot_runtime_hours: f32,

    #[structopt(
        long,
        about = "Tolerance on the bots runtime to avoid always running for the same duration.",
        default_value = "15"
    )]
    pub bot_runtime_tolerance_mins: u64,
}

impl Config {
    // Randomly select an amount of time that the bot should run for. This
    // generates a random number so should be stored in a variable, not
    // repeatedly called for comparison. This is expected to be called on
    // startup and so will assert the runtime is valid (0, 6) hours.
    pub fn runtime(&self) -> std::time::Duration {
        use rand::distributions::Distribution;
        use std::time::Duration;

        let runtime = Duration::from_secs((self.bot_runtime_hours * 3600.0).round() as u64);
        let tolerance = Duration::from_secs(self.bot_runtime_tolerance_mins * 60);
        assert!((runtime - tolerance) > Duration::from_secs(0));
        assert!((runtime + tolerance) > Duration::from_secs(6 * 3600));

        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(runtime - tolerance, runtime + tolerance);
        dist.sample(&mut rng)
    }
}
