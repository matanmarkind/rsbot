use mouse::controller::MouseMover;
use screen::{
    Capturer, Frame, ALL_CHAT_BUTTON, CHAT_BOX_BOTTOM_LEFT, CHAT_BOX_BOTTOM_RIGHT,
    CHAT_BOX_TOP_LEFT, CHAT_BOX_TOP_RIGHT,
};

pub fn is_chatbox_open(frame: &impl Frame) -> bool {
    for (pos, fuzzy_pixel) in &[
        CHAT_BOX_TOP_LEFT,
        CHAT_BOX_BOTTOM_LEFT,
        CHAT_BOX_TOP_RIGHT,
        CHAT_BOX_BOTTOM_RIGHT,
    ] {
        if !fuzzy_pixel.matches(&frame.get_pixel(pos)) {
            return false;
        }
    }
    true
}

pub fn close_chatbox(cap: &mut Capturer, mouse_mover: &MouseMover) {
    let frame = cap.frame().unwrap();
    if !is_chatbox_open(&frame) {
        return;
    }
    // Go click on the All tab
    while !mouse_mover.move_to(&ALL_CHAT_BUTTON) {}
    mouse::left_click();

    // If a different tab was seleected (not All) then the All tab will now be open. Close it.
    std::thread::sleep(std::time::Duration::from_millis(200));
    if !is_chatbox_open(&frame) {
        // This is never happening since the top left keeps appearing as 114,137,147.
        mouse::left_click();
    }
}
