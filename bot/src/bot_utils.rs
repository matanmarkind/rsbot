mod chatbox {
    use screen::{locations, Capturer, Frame, FuzzyPixel};
    use userinput::InputBot;
    use util::*;

    const CHAT_BOX_TOP_LEFT: (Position, FuzzyPixel) = (
        locations::CHAT_BOX_TOP_LEFT,
        FuzzyPixel {
            blue_min: 114,
            blue_max: 114,
            green_min: 137,
            green_max: 137,
            red_min: 147,
            red_max: 147,
        },
    );
    const CHAT_BOX_BOTTOM_LEFT: (Position, FuzzyPixel) = (
        locations::CHAT_BOX_BOTTOM_LEFT,
        FuzzyPixel {
            blue_min: 147,
            blue_max: 147,
            green_min: 169,
            green_max: 169,
            red_min: 173,
            red_max: 173,
        },
    );

    const CHAT_BOX_TOP_RIGHT: (Position, FuzzyPixel) = (
        locations::CHAT_BOX_TOP_RIGHT,
        FuzzyPixel {
            blue_min: 94,
            blue_max: 94,
            green_min: 112,
            green_max: 112,
            red_min: 119,
            red_max: 119,
        },
    );

    const CHAT_BOX_BOTTOM_RIGHT: (Position, FuzzyPixel) = (
        locations::CHAT_BOX_BOTTOM_RIGHT,
        FuzzyPixel {
            blue_min: 140,
            blue_max: 140,
            green_min: 154,
            green_max: 154,
            red_min: 162,
            red_max: 162,
        },
    );
    fn is_chatbox_open(frame: &impl Frame) -> bool {
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

    pub fn close_chatbox(cap: &mut Capturer, inputbot: &mut InputBot) {
        let frame = cap.frame().unwrap();
        if !is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        while !inputbot.move_near(&locations::ALL_CHAT_BUTTON) {}
        inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        let frame = cap.frame().unwrap();

        // If the chatbox is still open it's possible a different chat tab was
        // selected and now the ALL tab is on.
        if !is_chatbox_open(&frame) {
            return;
        }
        // Go click on the All tab
        while !inputbot.move_near(&locations::ALL_CHAT_BUTTON) {}
        inputbot.left_click();
        std::thread::sleep(REDRAW_TIME);
        let frame = cap.frame().unwrap();

        // If the chatbox is still open this is likely due to an update such as
        // leveling up. This closes by left clicking most things
        if !is_chatbox_open(&frame) {
            return;
        }
        // Click the center of the minimap since this will only move us a small
        // amount. Safest/easiest way I could think of torandomly left click.
        while !inputbot.move_near(&locations::MINIMAP_MIDDLE) {}
        inputbot.left_click();
    }
}

pub use chatbox::close_chatbox;