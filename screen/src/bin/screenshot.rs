// From scrap github repo. Here for my convenience.
use screen::*;
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use util::*;

fn main() {
    let mut capturer = screen::Capturer::new();

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
                    thread::sleep(FRAME_PERIOD);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        }
    }
    let subframe = frame.subframe(Position { x: 100, y: 100 }, Position { x: 500, y: 500 });
    dbg!(&subframe.width, &subframe.height, &subframe.len());

    println!("Captured! Saving...");
    repng::encode(
        File::create("screenshot.png").unwrap(),
        subframe.width as u32,
        subframe.height as u32,
        subframe.buffer(),
    )
    .unwrap();

    // Flip the BGRA image into a RGBA image.
    println!("Flipping...");

    let flipped = subframe.flip();

    // Save the image.
    println!("Saving Flipped...");
    repng::encode(
        File::create("screenshot_flipped.png").unwrap(),
        flipped.width as u32,
        flipped.height as u32,
        flipped.buffer(),
    )
    .unwrap();
}
