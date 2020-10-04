use device_query::{DeviceQuery, DeviceState};
/// Gets the pixel corresponding to the mouse's location and prints it.
use screen::Frame;
use screen::*;
use std::io::ErrorKind::WouldBlock;
use std::thread::sleep;
use util::*;

fn main() {
    // Take a screenshot.
    let mut capturer = Capturer::new();
    let frame;
    loop {
        // Wait until there's a frame.
        match capturer.frame() {
            Ok(f) => {
                frame = f;
                break;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    sleep(FRAME_PERIOD);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        }
    }

    // Get the mouse's location.
    let device_state = DeviceState::new();
    let (x, y) = device_state.get_mouse().coords;

    println!(
        "mouse=({}, {}), pixel={:?}",
        x,
        y,
        frame.get_pixel(&Position { x, y }),
    );
}
