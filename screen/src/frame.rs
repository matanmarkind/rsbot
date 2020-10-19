/// This file holds frames, which are t base a list of u8's describing an image.
///
/// The two types of frames are owned and unowned. This is due to scrap, which
/// returns unowned frames (due to xll), for I assume efficiency. The reason we
/// want to keep this, is that when the bot is running we don't need to do any
/// owning activities. For the sake of feedback it can be useful to mark up and
/// save the image, which requires ownership.
use crate::action_letters;
use crate::constants::*;
use crate::types::*;
use crate::Locations;
use std::cmp::{max, min};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::io::Result;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use util::*;

// When searching for a pixel in a frame, how many attempts to make.
pub const TIME_TO_FIND_PIXEL: Duration = Duration::from_millis(100);

pub type DefaultFrame<'a> = UnownedFrame<scrap::Frame<'a>>;

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
    fn check_loose_pixel_explicit(
        &self,
        pos: &Position,
        expected_pixel: &FuzzyPixel,
        tolerance: i32,
    ) -> bool {
        // dbg!(pos, expected_pixel);
        for x_shift in -tolerance..=tolerance {
            for y_shift in -tolerance..=tolerance {
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
    fn check_loose_pixel(&self, pos: &Position, expected_pixel: &FuzzyPixel) -> bool {
        self.check_loose_pixel_explicit(pos, expected_pixel, /*tolerance=*/ 1)
    }

    /// Search for a matching pixel in the bounds given bounds. This
    fn find_pixel_random(
        &self,
        fuzzy_pixel: &FuzzyPixel,
        top_left: &Position,
        dimensions: &DeltaPosition,
    ) -> Option<Position> {
        let batch_size = 1000;
        let time = std::time::Instant::now();
        while time.elapsed() < TIME_TO_FIND_PIXEL {
            // To avoid wasting time by always checking the time, only check
            // every 1k searches.
            for _ in 0..batch_size {
                let pos = random_position(top_left, dimensions);
                if fuzzy_pixel.contains(&self.get_pixel(&pos)) {
                    return Some(pos);
                }
            }
        }
        None
    }

    /// Search for a matching pixel in the bounds given bounds. Bounds are given
    /// in polar coordinates.
    fn find_pixel_random_polar(
        &self,
        fuzzy_pixel: FuzzyPixel,
        middle: Position,
        radius: f32,
    ) -> Option<Position> {
        let batch_size = 1000;
        let time = std::time::Instant::now();
        while time.elapsed() < TIME_TO_FIND_PIXEL {
            // To avoid wasting time by always checking the time, only check
            // every 1k searches.
            for _ in 0..batch_size {
                let pos = random_position_polar(middle, radius);
                if fuzzy_pixel.contains(&self.get_pixel(&pos)) {
                    return Some(pos);
                }
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
        // dbg!(fpath);
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

// TODO: Switch from consuming and returning self, to inputing &mut self and
// outputing &mut self.
impl OwnedFrame {
    /// Take a subframe. This means a frame contained within the frame in
    /// this object's buffer.
    ///
    /// 'top_left' - top left corner of the image (included). (x,y) represent
    /// the top/leftmost row/column from the original image that will be copied
    /// over.
    ///
    /// 'dimensions' - size of resultant frame.
    pub fn crop(&mut self, top_left: Position, dimensions: DeltaPosition) -> &mut OwnedFrame {
        let past_bottom_right = &top_left + &dimensions;
        assert!(past_bottom_right.x as usize <= self.width());
        assert!(past_bottom_right.y as usize <= self.height());

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

        self.width = dimensions.dx as usize;
        self.height = dimensions.dy as usize;
        self
    }

    /// Flip the image from either BGRA to RGBA or back. Always sets alpha to
    /// 255.
    pub fn flip(&mut self) -> &mut OwnedFrame {
        // Copy in each row segment.
        for pixel_offset in (0..self.buffer.len()).step_by(RAW_PIXEL_SIZE) {
            // Swap the blue and red channels.
            self.buffer.swap(pixel_offset, pixel_offset + 2);

            // Turn alpha to 255. This is to make the image visible.
            self.buffer[pixel_offset + 3] = 255;
        }

        self.is_bgr = !self.is_bgr;
        self
    }
    pub fn flip_to_rgb(&mut self) {
        if self.is_bgr {
            self.flip();
        }
    }
    pub fn flip_to_bgr(&mut self) {
        if !self.is_bgr {
            self.flip();
        }
    }

    pub fn recolor_pixel(&mut self, pos: &Position, color: &Pixel) {
        let pixel_offset = self.pixel_index(pos);

        self.buffer[pixel_offset] = if self.is_bgr { color.blue } else { color.red };
        self.buffer[pixel_offset + 1] = color.green;
        self.buffer[pixel_offset + 2] = if self.is_bgr { color.red } else { color.blue };
    }

    pub fn draw_vertical_line(&mut self, top: &Position, len: i32, line_color: &Pixel) {
        for i in 0..len {
            self.recolor_pixel(&(top + &Position { x: 0, y: i }), line_color);
        }
    }
    pub fn draw_horizontal_line(&mut self, top: &Position, len: i32, line_color: &Pixel) {
        for i in 0..len {
            self.recolor_pixel(&(top + &Position { x: i, y: 0 }), line_color);
        }
    }
    // Drow a box from 'top_left' (included).
    pub fn draw_box(
        &mut self,
        top_left: &Position,
        dimensions: &DeltaPosition,
        line_color: &Pixel,
    ) {
        self.draw_horizontal_line(top_left, dimensions.dx, line_color);
        self.draw_horizontal_line(
            &Position {
                x: top_left.x,
                y: top_left.y + dimensions.dy - 1,
            },
            dimensions.dx,
            line_color,
        );
        self.draw_vertical_line(top_left, dimensions.dy, line_color);
        self.draw_vertical_line(
            &Position {
                x: top_left.x + dimensions.dx - 1,
                y: top_left.y,
            },
            dimensions.dy,
            line_color,
        );
    }

    pub fn draw_red_box(&mut self, top_left: &Position, dimensions: &DeltaPosition) {
        self.draw_box(
            top_left,
            dimensions,
            &Pixel {
                blue: 0,
                green: 0,
                red: 255,
            },
        );
    }
}

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
    pub fn frame(&mut self) -> Result<DefaultFrame> {
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

/// Helper for getting information about frames.
///
/// The frame class is a basic class that shouldn't be especially tied to
/// runescape. This class layers on more functionality that is directly tied to
/// game related questions instead of image handling.
///
/// Importantly this class has state, Locations. This is configured on startup
/// based on where the game window is. The game window is never expected to move
/// during play.
pub struct FrameHandler {
    pub locations: crate::Locations,
    // Tolerance when checking loose pixels in the inventory. Having the bank
    // open seems to change things a bit.
    pub inventory_tolerance: i32,
}

impl FrameHandler {
    pub fn new(config: crate::Config) -> FrameHandler {
        FrameHandler {
            locations: Locations::new(
                config.screen_top_left,
                util::DeltaPosition {
                    dx: config.screen_bottom_right.x - config.screen_top_left.x + 1,
                    dy: config.screen_bottom_right.y - config.screen_top_left.y + 1,
                },
            ),
            inventory_tolerance: 1,
        }
    }

    pub fn check_action_letters(
        &self,
        frame: &impl Frame,
        letter_and_pixels: &[(action_letters::Letter, FuzzyPixel)],
    ) -> bool {
        action_letters::check_action_letters(
            frame,
            letter_and_pixels,
            self.locations.action_text_top_left(),
        )
    }
    pub fn mark_letters_and_save(
        &self,
        frame: &impl Frame,
        fpath: &str,
        letter_and_pixels: &[(action_letters::Letter, crate::FuzzyPixel)],
    ) -> std::thread::JoinHandle<()> {
        action_letters::mark_letters_and_save(
            frame,
            fpath,
            &letter_and_pixels,
            self.locations.action_text_top_left(),
        )
    }

    pub fn is_inventory_open(&self, frame: &impl Frame) -> bool {
        // Use check_loose_pixel because the background color of the icons is very
        // distinct between on and off and the satchel depicted is also a
        // significantly different color. If the image shifts, which it sometimes
        // does I don't want to be too brittle since I think the risk of a false
        // positive is relatively low.
        frame.check_loose_pixel(
            &self.locations.inventory_icon_background(),
            &colors::INVENTORY_ICON_BACKGROUND_OPEN,
        )
    }

    /// Based on Locations::INVENTORY_SLOT_CHECK_SPACING we perform 12 checks
    /// per inventory slot. This is constant across different screen sizes. This
    /// can switch to [(DeltaPosition, FuzzyPixel)] if relying on this constant
    /// becomes a problem.
    ///
    /// Make sure that the mouse is not hovering over the inventory since this
    /// causes text to appear messing up the frame.
    pub fn check_inventory_slot(
        &self,
        frame: &impl Frame,
        slot_index: i32,
        expected_colors: &[FuzzyPixel; Locations::NUM_CHECKS_PER_INVENTORY_SLOT],
    ) -> bool {
        let top_left = self.locations.inventory_slot_top_left(slot_index);
        let dimensions = self.locations.inventory_slot_dimensions();

        let past_bottom_right = &top_left + &dimensions;
        let check_spacing = Locations::INVENTORY_SLOT_CHECK_SPACING;

        // Don't bother checking the border between slots.
        let first_pos = &top_left + &check_spacing;
        let mut pos = first_pos;
        let mut i = 0;
        while pos.y < past_bottom_right.y {
            while pos.x < past_bottom_right.x {
                {
                    // let pixel = frame.get_pixel(&pos);
                    // let dbgstr = format!(
                    //     "slot_index={}, {:?}, {:?} {:?}",
                    //     slot_index, pos, pixel, expected_colors[i]
                    // );
                    // dbg!(dbgstr);
                }
                if !frame.check_loose_pixel_explicit(
                    &pos,
                    &expected_colors[i],
                    self.inventory_tolerance,
                ) {
                    return false;
                }
                pos = Position {
                    x: pos.x + check_spacing.dx,
                    y: pos.y,
                };
                i += 1;
            }
            pos = Position {
                x: first_pos.x,
                y: pos.y + check_spacing.dy,
            };
        }
        true
    }

    pub fn is_inventory_slot_open(&self, frame: &impl Frame, slot_index: i32) -> bool {
        self.check_inventory_slot(frame, slot_index, &colors::INVENTORY_SLOT_EMPTY)
    }

    /// Get the minimum slot_index [0,NUM_INVENTORY_SLOTS) which points to a
    /// matching inventory slot. Returns None if there is no open slot.
    pub fn first_matching_inventory_slot(
        &self,
        frame: &impl Frame,
        expected_colors: &[FuzzyPixel; Locations::NUM_CHECKS_PER_INVENTORY_SLOT],
    ) -> Option<i32> {
        for i in 0..Locations::NUM_INVENTORY_SLOTS {
            if self.check_inventory_slot(frame, i, expected_colors) {
                return Some(i);
            }
        }
        None
    }

    pub fn first_open_inventory_slot(&self, frame: &impl Frame) -> Option<i32> {
        self.first_matching_inventory_slot(frame, &colors::INVENTORY_SLOT_EMPTY)
    }

    /// Check the 4 corners of the box described by (top_left, dimensions)
    /// loosely matches expectations.
    ///
    /// expected_pixels are expected to be in the order [top_left, bottom_left,
    /// top_right, bottom_right]
    fn check_corners(
        frame: &impl Frame,
        top_left: Position,
        dimensions: DeltaPosition,
        expected_pixels: [FuzzyPixel; 4],
    ) -> bool {
        let pos_and_pixel = [
            (top_left, expected_pixels[0]),
            (
                Locations::to_bottom_left(top_left, dimensions),
                expected_pixels[1],
            ),
            (
                Locations::to_top_right(top_left, dimensions),
                expected_pixels[2],
            ),
            (
                Locations::to_bottom_right(top_left, dimensions),
                expected_pixels[3],
            ),
        ];
        for (pos, fuzzy_pixel) in &pos_and_pixel {
            // let pixel = frame.get_pixel(pos);
            // dbg!(pos, fuzzy_pixel, pixel);
            if !frame.check_loose_pixel(&pos, &fuzzy_pixel) {
                return false;
            }
        }
        true
    }

    /// Checks if the chatbox is open by chacking the corners. The coloring
    /// changes depending on whether or not runelite is the active window. We
    /// program on the assumption it is.
    pub fn is_chatbox_open(&self, frame: &impl Frame) -> bool {
        Self::check_corners(
            frame,
            self.locations.chatbox_inner_top_left(),
            self.locations.chatbox_inner_dimensions(),
            [
                FuzzyPixel {
                    blue_min: 63,
                    blue_max: 71,
                    green_min: 76,
                    green_max: 85,
                    red_min: 84,
                    red_max: 93,
                },
                FuzzyPixel {
                    blue_min: 81,
                    blue_max: 90,
                    green_min: 99,
                    green_max: 109,
                    red_min: 108,
                    red_max: 119,
                },
                FuzzyPixel {
                    blue_min: 83,
                    blue_max: 87,
                    green_min: 103,
                    green_max: 107,
                    red_min: 112,
                    red_max: 116,
                },
                FuzzyPixel {
                    blue_min: 115,
                    blue_max: 119,
                    green_min: 143,
                    green_max: 147,
                    red_min: 155,
                    red_max: 159,
                },
            ],
        )
    }

    /// Checks if the worldmap is open by chacking the corners. The coloring
    /// changes depending on whether or not runelite is the active window. We
    /// program on the assumption it is.
    pub fn is_worldmap_open(&self, frame: &impl Frame) -> bool {
        // The worldmap dimensions are internal, which means the colors are
        // variable (top left can be covered by action text, right size is on
        // the map.) Creating an outer barier would put us outside the screen.
        // Therefore we take the inner box and expand t a bit to rest on the
        // worldmap border which we can use to identify the worldmap being open.
        let expansion = DeltaPosition { dx: 3, dy: 3 };
        let top_left = self.locations.worldmap_top_left() - expansion;
        let dimensions = self.locations.worldmap_dimensions() + expansion * 2.0;
        Self::check_corners(
            frame,
            top_left,
            dimensions,
            [
                FuzzyPixel {
                    blue_min: 51,
                    blue_max: 62,
                    green_min: 55,
                    green_max: 66,
                    red_min: 54,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 51,
                    blue_max: 62,
                    green_min: 55,
                    green_max: 66,
                    red_min: 54,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 58,
                    blue_max: 62,
                    green_min: 62,
                    green_max: 66,
                    red_min: 61,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 249,
                    blue_max: 255,
                    green_min: 249,
                    green_max: 255,
                    red_min: 250,
                    red_max: 255,
                },
            ],
        )
    }

    /// Checks if the bank is open by chacking the corners. The coloring
    /// changes depending on whether or not runelite is the active window. We
    /// program on the assumption it is.
    pub fn is_bank_open(&self, frame: &impl Frame) -> bool {
        // The bank dimensions are internal, which means the colors are
        // variable (top left can be covered by action text, right size is on
        // the map.) Creating an outer barier would put us outside the screen.
        // Therefore we take the inner box and expand t a bit to rest on the
        // worldmap border which we can use to identify the worldmap being open.
        let expansion = DeltaPosition { dx: 3, dy: 3 };
        let top_left = self.locations.bank_top_left() - expansion;
        let dimensions = self.locations.bank_dimensions() + expansion * 2.0;
        Self::check_corners(
            frame,
            top_left,
            dimensions,
            [
                FuzzyPixel {
                    blue_min: 51,
                    blue_max: 62,
                    green_min: 55,
                    green_max: 66,
                    red_min: 54,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 51,
                    blue_max: 62,
                    green_min: 55,
                    green_max: 66,
                    red_min: 54,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 58,
                    blue_max: 62,
                    green_min: 62,
                    green_max: 66,
                    red_min: 61,
                    red_max: 65,
                },
                FuzzyPixel {
                    blue_min: 58,
                    blue_max: 62,
                    green_min: 62,
                    green_max: 66,
                    red_min: 61,
                    red_max: 65,
                },
            ],
        )
    }

    pub fn is_bank_quantity_all(&self, frame: &impl Frame) -> bool {
        frame.check_loose_pixel(
            &self.locations.bank_quantity_all(),
            &colors::BANK_QUANTITY_ON,
        )
    }
    pub fn is_bank_quantity_one(&self, frame: &impl Frame) -> bool {
        frame.check_loose_pixel(
            &self.locations.bank_quantity_one(),
            &colors::BANK_QUANTITY_ON,
        )
    }
}
