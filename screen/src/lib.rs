use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::io::ErrorKind::WouldBlock;
use std::io::Result;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use util::*;

// Each pixel is represented by 4 u8's, BGRA. Each frame is a list of u8's.
pub const PIXEL_SIZE: usize = 4;

// Amount of time to wait between attempts to capture a Frame.
pub const FRAME_PERIOD: Duration = Duration::from_micros(1e6 as u64 / 60);

const ATTEMPTS_TO_FIND_PIXEL: u32 = 10000;

/// Search the screen for a desired pixel.
///
/// 'capturer' - used to take a screenshot.
///
/// 'desired_bgr_pixels' - set of pixels to match against (aka find within the frame).
///
/// 'top_left' - top left corner of the image (included). (x,y) represent the
/// top/leftmost row/column of the frame to search in.
///
/// 'past_bottom_right' - bottom right of the image (excluded). (x,y) represent
/// one past the bottom/rightmost row/column of the frame to search in.
///
/// Returns the position of the first pixel found which matches the criteria. If
/// no pixel is found return None.
pub fn find_pixel_exact(
    capturer: &mut Capturer,
    desired_bgr_pixels: &HashSet<(u8, u8, u8)>,
    top_left: &Position,
    past_bottom_right: &Position,
) -> Option<Position> {
    // Get a frame.
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

    for _ in 0..ATTEMPTS_TO_FIND_PIXEL {
        // Randomly generate positions in the provided range.
        let position = random_position(top_left, past_bottom_right);

        // Get the BGR pixel from the frame at this Position.
        let pixel = get_pixel_from_frame(&frame, &position);

        if desired_bgr_pixels.contains(&pixel) {
            return Some(position);
        }
    }

    None
}

/// Search the screen for a desired pixel.
///
/// 'capturer' - used to take a screenshot.
///
/// 'bgr_pixels' - set of pixels ranges to match against (aka find within the
/// frame). Each element is a pair of [min, max] to apply to each channel of the
/// pixel.
///
/// 'top_left' - top left corner of the image (included). (x,y) represent the
/// top/leftmost row/column of the frame to search in.
///
/// 'past_bottom_right' - bottom right of the image (excluded). (x,y) represent
/// one past the bottom/rightmost row/column of the frame to search in.
///
/// Returns the position of the first pixel found which matches the criteria. If
/// no pixel is found return None.
pub fn find_pixel_fuzzy(
    capturer: &mut Capturer,
    desired_bgr_pixel: &((u8, u8), (u8, u8), (u8, u8)),
    top_left: &Position,
    past_bottom_right: &Position,
) -> Option<Position> {
    // Get a frame.
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

    for _ in 0..ATTEMPTS_TO_FIND_PIXEL {
        // Randomly generate positions in the provided range.
        let position = random_position(top_left, past_bottom_right);

        // Get the BGR pixel from the frame at this Position.
        let pixel = get_pixel_from_frame(&frame, &position);

        // Compiles without the parents around desired_bgr_pixel.X, but cargo
        // fmt doesn't work.
        if pixel.0 >= (desired_bgr_pixel.0).0
            && pixel.0 <= (desired_bgr_pixel.0).1
            && pixel.1 >= (desired_bgr_pixel.1).0
            && pixel.1 <= (desired_bgr_pixel.1).1
            && pixel.2 >= (desired_bgr_pixel.2).0
            && pixel.2 <= (desired_bgr_pixel.2).1
        {
            return Some(position);
        }
    }

    None
}

/// 'top_left' - top left corner of the image (included). (x,y) represent the
/// top/leftmost row/column of the frame to search in.
///
/// 'past_bottom_right' - bottom right of the image (excluded). (x,y) represent
/// one past the bottom/rightmost row/column of the frame to search in.
///
/// Returns the position of the first pixel found which matches the criteria. If
/// no pixel is found return None.
fn random_position(top_left: &Position, past_bottom_right: &Position) -> Position {
    let mut rng = thread_rng();
    Position {
        x: rng.gen_range(top_left.x, past_bottom_right.x),
        y: rng.gen_range(top_left.y, past_bottom_right.y),
    }
}

// Frame must conform to PIXEL_SIZE (4 u8 elements per pixel with the first 3
// being the ones of interest)
fn get_pixel_from_frame<DataT>(frame: &BgrFrame<DataT>, position: &Position) -> (u8, u8, u8)
where
    DataT: Deref<Target = [u8]>,
{
    let row_offset = frame.width() * PIXEL_SIZE * position.y as usize;
    let pixel_offset = row_offset + position.x as usize * PIXEL_SIZE;
    return (
        frame.buffer()[pixel_offset],
        frame.buffer()[pixel_offset + 1],
        frame.buffer()[pixel_offset + 2],
    );
}

// FrameT should be some wrapper to [u8], which is a vector of pixels ordered by row.
pub struct Frame<DataT>
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
    /// Take a subframe. This means a frame contained within the frame in
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

pub struct BgrFrame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    pub frame: Frame<DataT>,
}

pub struct RgbFrame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    pub frame: Frame<DataT>,
}

impl<DataT> BgrFrame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    pub fn new(buffer: DataT, width: usize, height: usize) -> BgrFrame<DataT> {
        BgrFrame {
            frame: Frame {
                buffer: buffer,
                width: width,
                height: height,
            },
        }
    }
    pub fn subframe(&self, top_left: Position, past_bottom_right: Position) -> BgrFrame<Vec<u8>> {
        BgrFrame {
            frame: self.frame.subframe(top_left, past_bottom_right),
        }
    }

    /// Flip the image from either BGRA to RGBA or back. Always sets alpha to
    /// 255.
    ///
    /// TODO: consider turning this into a mutating function, where it flips its
    /// own elements.
    pub fn flip(&self) -> RgbFrame<Vec<u8>> {
        RgbFrame {
            frame: self.frame.flip(),
        }
    }

    pub fn width(&self) -> usize {
        self.frame.width
    }
    pub fn height(&self) -> usize {
        self.frame.height
    }
    pub fn len(&self) -> usize {
        self.frame.buffer.len()
    }
    pub fn buffer(&self) -> &[u8] {
        &self.frame.buffer[..]
    }
}

impl<DataT> RgbFrame<DataT>
where
    DataT: Deref<Target = [u8]>,
{
    pub fn new(buffer: DataT, width: usize, height: usize) -> RgbFrame<DataT> {
        RgbFrame {
            frame: Frame {
                buffer: buffer,
                width: width,
                height: height,
            },
        }
    }
    pub fn subframe(&self, top_left: Position, past_bottom_right: Position) -> RgbFrame<Vec<u8>> {
        RgbFrame {
            frame: self.frame.subframe(top_left, past_bottom_right),
        }
    }

    /// Flip the image from either BGRA to RGBA or back. Always sets alpha to
    /// 255.
    ///
    /// TODO: consider turning this into a mutating function, where it flips its
    /// own elements.
    pub fn flip(&self) -> RgbFrame<Vec<u8>> {
        RgbFrame {
            frame: self.frame.flip(),
        }
    }

    pub fn width(&self) -> usize {
        self.frame.width
    }

    pub fn height(&self) -> usize {
        self.frame.height
    }
    pub fn len(&self) -> usize {
        self.frame.buffer.len()
    }

    pub fn buffer(&self) -> &[u8] {
        &self.frame.buffer[..]
    }
}

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
        assert_eq!(width * height * PIXEL_SIZE, frame.len());

        capturer
    }

    // TODO: Once I can compile with polonius, switch to '_unused_frame'.
    pub fn frame(&mut self) -> Result<BgrFrame<scrap::Frame>> {
        let (width, height) = (self.capturer.width(), self.capturer.height());
        // Wait until there's a frame.
        match self.capturer.frame() {
            Ok(frame) => Ok(BgrFrame::new(frame, width, height)),
            Err(err) => Err(err),
        }
    }

    /*
    /// polonius is causing an imported crate (uinput) not to compile, so this
    /// implementation is off the table for now :(.
    ///
    /// Returns a screenshot of the primary display as [u8] in BGRA mode. Will
    /// block until a screencapture is achieved. May panic.
    pub fn _unused_frame(&mut self) -> Frame<scrap::Frame> {
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
