pub mod constants;
pub mod controller;
pub mod types;

pub fn left_click() {
    use crate::constants::*;
    use inputbot::MouseButton::LeftButton;
    // TODO: Consider moving uniform to normal distribution.
    use rand::distributions::{Distribution, Uniform};

    let mut rng = rand::thread_rng();
    let duration = Uniform::new(MIN_CLICK_WAIT, MAX_CLICK_WAIT);

    LeftButton.press();
    std::thread::sleep(duration.sample(&mut rng));
    LeftButton.release();
}

pub fn left_arrow(press_time: std::time::Duration) {
    use inputbot::KeybdKey::AKey;
    // TODO: Consider moving uniform to normal distribution.
    use rand::distributions::{Distribution, Uniform};

    // It seems that scan codes differentiate between the left arrow key and the
    // left number bad key (4).
    // - Inputbot seems to give the number bad version which OSRS doesn't care
    //   for.
    //   https://github.com/obv-mikhail/InputBot/blob/32a7d5e150753a5f7eefbe06fbef9b2f4015c876/src/linux/inputs.rs
    // - To see scancodes - sudo showkey -s
    // 1. Left arrow code - 0xe0 0x4b 0xe0 0xcb
    // 2. Left number pad (4) 0x4b 0xcb
    // The solution is to use runelight's "Key Remapping" plugin so that A is left and D is right.

    let mut rng = rand::thread_rng();
    let slack = std::time::Duration::from_millis(20);
    let duration = Uniform::new(press_time - slack, press_time + slack);
    println!("press");

    let time = std::time::Instant::now();
    while time.elapsed() < press_time {
        AKey.press();
        // It seems like cycling every ms is good. Gives us nearly normal speed.
        // Going down too slow is innefficient and choppy and probably doesn't
        // look very human. Going too fast creates a backlog to the screen keeps
        // circling.
        std::thread::sleep(std::time::Duration::from_millis(1));
        AKey.release();
    }
    println!("release");
}
