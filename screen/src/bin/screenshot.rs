// From scrap github repo. Here for my convenience.

use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (width, height) = (capturer.width(), capturer.height());
    let w = width / 2;
    let h = height / 2;

    loop {
        // Wait until there's a frame.

        let raw_buffer = match capturer.frame() {
            Ok(raw_buffer) => raw_buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        dbg!(raw_buffer.len(), w * h * 4);

        let mut buffer = Vec::with_capacity(w * h * 4);
        let stride = raw_buffer.len() / height;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                buffer.extend_from_slice(&raw_buffer[i..=(i + 3)]);
            }
        }

        println!("Captured! Saving...");
        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &buffer,
        )
        .unwrap();

        // Flip the BGRA image into a RGBA image.
        println!("Flipping...");

        let mut bitflipped = Vec::with_capacity(buffer.len());
        let stride = buffer.len() / h;
        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }

        // Save the image.
        println!("Saving Flipped...");
        repng::encode(
            File::create("screenshot_flipped.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        )
        .unwrap();

        break;
    }
}
