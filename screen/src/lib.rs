pub mod action_letters;
pub mod capture;
pub mod constants;
pub mod frame;
pub mod inventory;
pub mod types;

/// Publish internals so users can use screen::Frame instead of
/// screen::frame::Frame.
pub use action_letters::*;
pub use capture::*;
pub use constants::*;
pub use frame::*;
pub use types::*;

pub fn mark_letters_and_save(
    frame: &impl crate::Frame,
    fpath: &str,
    letter_and_matchers: &[(ActionLetter, FuzzyPixel)],
) {
    let mut img = frame.to_owned();

    let mut x_offset = locations::TOP_LEFT_ACTION_TEXT.x;
    for (letter, _) in letter_and_matchers {
        for util::DeltaPosition { dx, dy } in letter.checkpoints.iter() {
            let pos = util::Position {
                x: x_offset + dx,
                y: locations::TOP_LEFT_ACTION_TEXT.y + dy,
            };
            img.recolor_pixel(&pos, &colors::PURE_RED);
        }
        x_offset += letter.width;
    }

    // Spawn image saving to another thread since it takes a very long time.
    let fpath = fpath.to_string();
    std::thread::spawn(move || {
        img.crop(
            locations::WINDOW_TOP_LEFT,
            &locations::WINDOW_TOP_LEFT + &locations::WINDOW_DIMENSIONS,
        )
        .flip_to_rgb();
        img.save(fpath.as_str());
    });
}
