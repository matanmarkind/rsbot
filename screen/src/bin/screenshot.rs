// From scrap github repo. Here for my convenience.

use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::ops::Deref;
use std::thread;
use std::time::Duration;
use util::*;

// Each pixel is represented by 4 u8's, BGRA.
const PIXEL_SIZE: usize = 4;

struct Capturer {
    capturer: scrap::Capturer,

    // Amount of time to wait between attempting to capture a frame.
    frame_period: Duration,
}

impl Capturer {
    fn new() -> Capturer {
        let display = scrap::Display::primary().expect("Couldn't find primary display.");
        let mut capturer = Capturer {
            capturer: scrap::Capturer::new(display).expect("Couldn't begin capture."),
            frame_period: Duration::from_secs(1) / 60,
        };

        let (width, height) = (capturer.width(), capturer.height());
        let frame = capturer.frame();
        assert_eq!(width * height * PIXEL_SIZE, frame.len());

        capturer
    }

    /// Will block until a screencapture is achieved. May panic.
    fn frame(&mut self) -> scrap::Frame {
        // This function requires compiling with:
        //
        //     RUSTFLAGS="-Zpolonius" cargo +nightly
        //
        // due to "mutable borrow starts here in previous iteration of loop"
        loop {
            // Wait until there's a frame.
            match self.capturer.frame() {
                Ok(frame) => return frame,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // Keep spinning.
                        thread::sleep(self.frame_period);
                        continue;
                    } else {
                        panic!("Error: {}", error);
                    }
                }
            }
        }
    }

    // Produces a new vector, not a new frame, since the data must be owned.
    // TODO: I think I could produce a Frame if I boxed the vector. May create a
    // simpler interface, but not sure it's worth it. Idk if I plan to do
    // subframe.subframe.
    fn subframe(frame: &scrap::Frame, top_left: Position, bottom_right: Position) -> Vec<u8> {
        let delta = &bottom_right - &top_left;
        let mut buffer = Vec::with_capacity(delta.dx * delta.dy * PIXEL_SIZE);
        // Copy in each row segment.

        buffer
    }

    // The width and height of the screen in pixels.
    fn width(&self) -> usize {
        self.capturer.width()
    }
    fn height(&self) -> usize {
        self.capturer.height()
    }

    // Each frame is represented as a [u8]. This is the number of elements used
    // to represent a row.
    fn row_len(&self) -> usize {
        self.width() * PIXEL_SIZE
    }
}

fn main() {
    let mut capturer = Capturer::new();
    let row_len = capturer.row_len();
    let (width, height) = (capturer.width(), capturer.height());
    let w = width / 2;
    let h = height / 2;

    // Wait until there's a frame.

    let raw_buffer = capturer.frame();
    dbg!(raw_buffer.len(), w * h * 4);

    let mut buffer = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let i = row_len * y + 4 * x;
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

    dbg!(util::Position { x: 100, y: 100 });
}
