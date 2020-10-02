pub mod constants;
pub mod controller;
pub mod types;

pub fn left_click() {
    use crate::constants::*;
    use inputbot::MouseButton::LeftButton;
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();
    let duration = Uniform::new(MIN_CLICK_WAIT, MAX_CLICK_WAIT);

    LeftButton.press();
    std::thread::sleep(duration.sample(&mut rng));
    LeftButton.release();
}
