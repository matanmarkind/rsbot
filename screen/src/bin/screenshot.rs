// From scrap github repo. Here for my convenience.
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::ops::Deref;
use std::thread;
use std::time::Duration;
use util::*;

// Each pixel is represented by 4 u8's, BGRA. Each frame is a list of u8's.
const PIXEL_SIZE: usize = 4;

// FrameT should be some wrapper to [u8], which is a vector of pixels ordered by row.
#[derive(Debug)]
struct Frame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    width: usize,
    height: usize,
    // DataT may represent an owned or unowned frame (Vec<u8> or scrap::Frame).
    buffer: DataT,
}

impl<DataT> Frame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    /// Take a subframe. This means a frame contained within the frame within
    /// this object's buffer.
    ///
    /// 'top_left' - top left corner of the image (included). (x,y) represent
    /// the top/leftmost row/column from the original image that will be copied
    /// over.
    ///
    /// 'past_bottom_right' - bottom right of the image (excluded). (x,y)
    /// represent one past the bottom/rightmost row/column from the original
    /// image the will be copied over.
    pub fn subframe(&self, top_left: Position, past_bottom_right: Position) -> Frame<Vec<u8>> {
        assert!(top_left.x < past_bottom_right.x);
        assert!(top_left.y < past_bottom_right.y);
        assert!(past_bottom_right.x as usize <= self.width);
        assert!(past_bottom_right.y as usize <= self.height);

        let delta = &past_bottom_right - &top_left;
        let mut buffer = Vec::with_capacity(delta.dx as usize * delta.dy as usize * PIXEL_SIZE);

        // Copy in each row segment.
        for row in top_left.y..past_bottom_right.y {
            let row_offset = self.width * PIXEL_SIZE * row as usize;
            let first = row_offset + top_left.x as usize * PIXEL_SIZE;
            let last = first + delta.dx as usize * PIXEL_SIZE;
            buffer.extend_from_slice(&self.buffer[first..last])
        }

        Frame {
            buffer: buffer,
            width: delta.dx as usize,
            height: delta.dy as usize,
        }
    }

    /// Flip the image from either BGRA to RGBA or back. Always sets alpha to
    /// 255.
    ///
    /// TODO: consider turning this into a mutating function, where it flips its
    /// own elements.
    pub fn flip(&self) -> Frame<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.buffer.len());

        // Copy in each row segment.
        for row in 0..self.height {
            for col in 0..self.width {
                let row_offset = self.width * PIXEL_SIZE * row as usize;
                let pixel_offset = row_offset + col as usize * PIXEL_SIZE;
                buffer.extend_from_slice(&[
                    self.buffer[pixel_offset + 2],
                    self.buffer[pixel_offset + 1],
                    self.buffer[pixel_offset],
                    255,
                ]);
            }
        }

        Frame {
            buffer: buffer,
            width: self.width,
            height: self.height,
        }
    }
}

struct Capturer {
    capturer: scrap::Capturer,

    // Amount of time to wait between attempting to capture a frame.
    frame_period: Duration,
}

impl Capturer {
    fn new() -> Capturer {
        let display = scrap::Display::primary().expect("Couldn't find primary display.");
        let cap = scrap::Capturer::new(display).expect("Couldn't begin capture.");
        let (width, height) = (cap.width(), cap.height());
        let mut capturer = Capturer {
            capturer: cap,
            frame_period: Duration::from_secs(1) / 60,
        };

        let frame = capturer.frame();
        assert_eq!(width * height * PIXEL_SIZE, frame.buffer.len());

        capturer
    }

    /// Returns a screenshot of the primary display as [u8] in BGRA mode.
    /// Will block until a screencapture is achieved. May panic.
    fn frame(&mut self) -> Frame<scrap::Frame> {
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
                    return Frame {
                        buffer: frame,
                        width,
                        height,
                    }
                }
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
}

fn main() {
    let mut capturer = Capturer::new();

    let frame = capturer.frame();
    let subframe = frame.subframe(Position { x: 100, y: 100 }, Position { x: 500, y: 500 });
    dbg!(&subframe.width, &subframe.height, &subframe.buffer.len());

    println!("Captured! Saving...");
    repng::encode(
        File::create("screenshot.png").unwrap(),
        subframe.width as u32,
        subframe.height as u32,
        &subframe.buffer,
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
        &flipped.buffer,
    )
    .unwrap();
}
