use crate::constants::*;
use crate::frame::*;
use std::io::ErrorKind::WouldBlock;
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

// Amount of time to wait between attempts to capture a Frame.
pub const FRAME_PERIOD: Duration = Duration::from_micros(1e6 as u64 / 60);

pub struct Capturer {
    pub capturer: scrap::Capturer,
}

impl Capturer {
    pub fn new() -> Capturer {
        let display = scrap::Display::primary().expect("Couldn't find primary display.");
        let capturer = scrap::Capturer::new(display).expect("Couldn't begin capture.");
        let (width, height) = (capturer.width(), capturer.height());
        let mut capturer = Capturer { capturer };

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
        assert_eq!(width * height * RAW_PIXEL_SIZE, frame.buffer().len());

        capturer
    }

    /// Takes a screenshot of the selected display and returns the BGRA frame.
    // TODO: Once I can compile with polonius, switch to '_unused_frame'.
    pub fn frame(&mut self) -> Result<UnownedFrame<scrap::Frame>> {
        let (width, height) = (self.capturer.width(), self.capturer.height());
        // Wait until there's a frame.
        match self.capturer.frame() {
            Ok(frame) => Ok(UnownedFrame {
                buffer: frame,
                width,
                height,
                is_bgr: true,
            }),
            Err(err) => Err(err),
        }
    }

    /*
    /// polonius is causing an imported crate (uinput) not to compile, so this
    /// implementation is off the table for now :(.
    ///
    /// Returns a screenshot of the primary display as [u8] in BGRA mode. Will
    /// block until a screencapture is achieved. May panic.
    pub fn _unused_frame(&mut self) -> UnownedFrame<scrap::Frame> {
        // This function requires compiling with:
        //
        //     RUSTFLAGS="-Zpolonius" cargo +nightly
        //
        // due to "mutable borrow starts here in previous iteration of loop"
        let (width, height) = (self.capturer.width(), self.capturer.height());
        loop {
            // Wait until there's a frame.
            match self.capturer.frame() {
                Ok(frame) => {
                    return UnownedFrame {
                        buffer: frame,
                        width,
                        height,
                        is_bgr: true,
                    }
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
    }
    */
}
