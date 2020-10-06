mod chatbox {
    use userinput::InputBot;
    use screen::{Capturer, Frame, FuzzyPixel, MINIMAP_MIDDLE, WINDOW_TOP_LEFT};
    use util::*;

    /// Chat buttons. Need to check them to make sure the chat box is closed.
    const ALL_CHAT_BUTTON: Position = Position {
        x: WINDOW_TOP_LEFT.x + 15,
        y: WINDOW_TOP_LEFT.y + 619,
    };
    // Pixel { blue: 35, green: 75, red: 98 }, false
    // Pixel { blue: 41, green: 51, red: 60 }, true
    // Pixel { blue: 35, green: 75, red: 98 }, false
    const ALL_CHAT_ON_HIGHLIGHTS: &[FuzzyPixel] = &[
        FuzzyPixel {
            blue_min: 39,
            blue_max: 42,
            green_min: 49,
            green_max: 52,
            red_min: 58,
            red_max: 61,
        },
        FuzzyPixel {
            blue_min: 34,
            blue_max: 36,
            green_min: 74,
            green_max: 76,
            red_min: 97,
            red_max: 99,
        },
    ];

    const CHAT_BOX_TOP_LEFT: (Position, FuzzyPixel) = (
        Position {
            x: WINDOW_TOP_LEFT.x + 10,
            y: WINDOW_TOP_LEFT.y + 471,
        },
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
        Position {
            x: WINDOW_TOP_LEFT.x + 5,
            y: WINDOW_TOP_LEFT.y + 604,
        },
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
        Position {
            x: WINDOW_TOP_LEFT.x + 518,
            y: WINDOW_TOP_LEFT.y + 473,
        },
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
        Position {
            x: WINDOW_TOP_LEFT.x + 520,
            y: WINDOW_TOP_LEFT.y + 604,
        },
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
        while !inputbot.move_near(&ALL_CHAT_BUTTON) {}
        inputbot.left_click();

        std::thread::sleep(REDRAW_TIME);
        let frame = cap.frame().unwrap();
        if !is_chatbox_open(&frame) {
            return;
        }

        // If the ALL chat tab is now open we should turn it off.
        let all_chat_pixel = frame.get_pixel(&ALL_CHAT_BUTTON);
        if !ALL_CHAT_ON_HIGHLIGHTS
            .iter()
            .any(|pixel| pixel.matches(&all_chat_pixel))
        {
            // Chatbox is open, but not due to a chat tab. Could be from a game
            // update, like leveling up which is shrunk by left clicking on the
            // game. Left click in the center of the MINI_MAP which will shrink the
            // chat tab without doing anything else.
            println!("Chat box open other.");
            while !inputbot.move_near(&MINIMAP_MIDDLE) {}
        }
        inputbot.left_click();

        let frame = cap.frame().unwrap();
        println!("is_chatbox_open={}", is_chatbox_open(&frame));
    }
}

pub use chatbox::close_chatbox;
