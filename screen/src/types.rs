use util::*;

/// When the mouse is placed over an object to act on, the top left of the
/// screen describes the action. We will "read" the action to confirm me want to
/// do that action.
///
/// These are all expected to be constants, so the lifetimes will be static.
pub struct ActionLetter<'a> {
    /// How wide is the letter, use to figure out the offset of the next letter.
    pub width: i32,

    /// Points checked to confirm this is the expected letter. Each element is
    /// given as the offset from the top_left of the box. The top is typically
    /// y=52. Letters are drawn in white, and the background can be of any
    /// color.
    pub checkpoints: &'a [DeltaPosition],
}
