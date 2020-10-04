/// This file holds frames, which are t base a list of u8's describing an image.
///
/// The two types of frames are owned and unowned. This is due to scrap, which
/// returns unowned frames (due to xll), for I assume efficiency. The reason we
/// want to keep this, is that when the bot is running we don't need to do any
/// owning activities. For the sake of feedback it can be useful to mark up and
/// save the image, which requires ownership.
use crate::constants::*;
use crate::types::*;
use std::cmp::{max, min};
use std::fs::File;
use std::ops::Deref;
use std::time::Duration;
use util::*;

// When searching for a pixel in a frame, how many attempts to make.
pub const TIME_TO_FIND_PIXEL: Duration = Duration::from_millis(100);

/// The interface for an unowned frame. Owned frames will also implement this.
pub trait Frame {
    /// These are the "fields" that frames will all have. This is to allow the
    /// trait to define default implementations for functions which are the same
    /// between different impls of Frame.

    /// Dimensions of the image in pixels. width * height * 4 = buffer.len.
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Is the frame in BGRA more or RGBA mode.
    fn is_bgr(&self) -> bool;
    /// Buffer of pixel channels. Buffer will contain info in either BGRA or
    /// RGBA format.
    fn buffer(&self) -> &[u8];

    /// Retrieve the pixel located at 'pos'.
    fn get_pixel(&self, pos: &Position) -> Pixel {
        let pixel_offset = self.pixel_index(pos);
        if self.is_bgr() {
            Pixel {
                blue: self.buffer()[pixel_offset],
                green: self.buffer()[pixel_offset + 1],
                red: self.buffer()[pixel_offset + 2],
            }
        } else {
            Pixel {
                blue: self.buffer()[pixel_offset + 2],
                green: self.buffer()[pixel_offset + 1],
                red: self.buffer()[pixel_offset],
            }
        }
    }

    // Check that one of the pixels around 'pos' (+-1) match the expectation.
    fn check_loose_pixel(&self, pos: &Position, expected_pixel: &FuzzyPixel) -> bool {
        for x_shift in [-1, 0, 1].iter() {
            for y_shift in [-1, 0, 1].iter() {
                let pos = Position {
                    x: min(max(0, pos.x + x_shift), self.width() as i32 - 1),
                    y: min(max(0, pos.y + y_shift), self.height() as i32 - 1),
                };
                if expected_pixel.matches(&self.get_pixel(&pos)) {
                    return true;
                }
            }
        }
        false
    }

    /// Search for a matching pixel in the bounds given bounds. This
    fn find_pixel_random(
        &self,
        fuzzy_pixel: &FuzzyPixel,
        top_left: &Position,
        past_bottom_right: &Position,
    ) -> Option<Position> {
        let time = std::time::Instant::now();
        while time.elapsed() < TIME_TO_FIND_PIXEL {
            let pos = random_position(top_left, past_bottom_right);
            if fuzzy_pixel.contains(&self.get_pixel(&pos)) {
                return Some(pos);
            }
        }
        None
    }

    /// Get the index to the first channel of the pixel as 'pos'.
    fn pixel_index(&self, pos: &Position) -> usize {
        self.width() * RAW_PIXEL_SIZE * pos.y as usize + pos.x as usize * RAW_PIXEL_SIZE
    }

    fn to_owned(&self) -> OwnedFrame {
        OwnedFrame {
            buffer: self.buffer().to_vec(),
            width: self.width(),
            height: self.height(),
            is_bgr: self.is_bgr(),
        }
    }

    fn save(&self, fpath: &str) {
        repng::encode(
            File::create(fpath).unwrap(),
            self.width() as u32,
            self.height() as u32,
            self.buffer(),
        )
        .unwrap();
    }
}

/// This represents a frame which does not own it's data. Will only implement the Frame trait.
pub struct UnownedFrame<BufferT>
where
    // AsRef should probably work for what I want in theory, but scrap::Frame only
    // implements Deref, so that's what I need to require.
    BufferT: Deref<Target = [u8]>,
{
    pub is_bgr: bool, // A frame can either be BGRA or RGBA.
    pub width: usize,
    pub height: usize,

    // BufferT may represent an owned or unowned frame (Vec<u8> or scrap::Frame).
    pub buffer: BufferT,
}

impl<BufferT> Frame for UnownedFrame<BufferT>
where
    BufferT: Deref<Target = [u8]>,
{
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    /// Is the frame in BGRA more or RGBA mode.
    fn is_bgr(&self) -> bool {
        self.is_bgr
    }
    /// Buffer of pixel channels. Buffer will contain info in either BGRA or
    /// RGBA format.
    /// TODO: try using .as_ref() instead.
    fn buffer(&self) -> &[u8] {
        &self.buffer[..]
    }
}

/// This represents a frame where the data is owned by the frame, meaning that
/// we can mutate it.
pub struct OwnedFrame {
    pub is_bgr: bool, // A frame can either be BGRA or RGBA.
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u8>,
}

impl Frame for OwnedFrame {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    /// Is the frame in BGRA more or RGBA mode.
    fn is_bgr(&self) -> bool {
        self.is_bgr
    }
    /// Buffer of pixel channels. Buffer will contain info in either BGRA or
    /// RGBA format.
    /// TODO: try using .as_ref() instead.
    fn buffer(&self) -> &[u8] {
        &self.buffer[..]
    }
}

impl OwnedFrame {
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
    pub fn crop(mut self, top_left: Position, past_bottom_right: Position) -> OwnedFrame {
        assert!(top_left.x < past_bottom_right.x);
        assert!(top_left.y < past_bottom_right.y);
        assert!(past_bottom_right.x as usize <= self.width());
        assert!(past_bottom_right.y as usize <= self.height());

        let delta = &past_bottom_right - &top_left;

        let mut i = 0;
        for row in top_left.y..past_bottom_right.y {
            for col in top_left.x..past_bottom_right.x {
                // The value copied will always be at a higher or equal index to the index it is overwriting. Since both are monotonically increasing once a value has been read out, we no longer need it to remain valid.
                let pixel_offset = self.pixel_index(&Position { x: col, y: row });
                self.buffer[i] = self.buffer[pixel_offset];
                self.buffer[i + 1] = self.buffer[pixel_offset + 1];
                self.buffer[i + 2] = self.buffer[pixel_offset + 2];
                self.buffer[i + 3] = self.buffer[pixel_offset + 3];
                i += RAW_PIXEL_SIZE;
            }
        }
        assert!(i <= self.buffer().len());
        self.buffer.resize(i, 0);

        OwnedFrame {
            buffer: self.buffer,
            width: delta.dx as usize,
            height: delta.dy as usize,
            is_bgr: self.is_bgr,
        }
    }

    /// Flip the image from either BGRA to RGBA or back. Always sets alpha to
    /// 255.
    pub fn flip(mut self) -> OwnedFrame {
        // Copy in each row segment.
        for pixel_offset in (0..self.buffer.len()).step_by(RAW_PIXEL_SIZE) {
            // Swap the blue and red channels.
            self.buffer.swap(pixel_offset, pixel_offset + 2);

            // Turn alpha to 255. This is to make the image visible.
            self.buffer[pixel_offset + 3] = 255;
        }

        OwnedFrame {
            buffer: self.buffer,
            width: self.width,
            height: self.height,
            is_bgr: !self.is_bgr,
        }
    }

    pub fn draw_vertical_line(&mut self, top: &Position, len: i32, line_color: &Pixel) {
        for i in 0..len {
            let pixel_offset = self.pixel_index(&(top + &Position { x: 0, y: i }));

            self.buffer[pixel_offset] = if self.is_bgr {
                line_color.blue
            } else {
                line_color.red
            };

            self.buffer[pixel_offset + 1] = line_color.green;

            self.buffer[pixel_offset + 2] = if self.is_bgr {
                line_color.red
            } else {
                line_color.blue
            };
        }
    }
}
