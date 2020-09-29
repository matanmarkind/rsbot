/// Gets the pixel corresponding to the mouse's location and prints it.
use device_query::{DeviceQuery, DeviceState};
use screen::*;
use std::io::ErrorKind::WouldBlock;
use std::thread::sleep;

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

    let row_offset = frame.width() * PIXEL_SIZE * y as usize;
    let pixel_offset = row_offset + x as usize * PIXEL_SIZE;
    println!(
        "mouse=({}, {}), pixel={:?}",
        x,
        y,
        frame.buffer()[pixel_offset..(pixel_offset + PIXEL_SIZE)].to_vec()
    );
}
