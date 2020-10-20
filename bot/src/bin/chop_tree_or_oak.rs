/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::{
    controller, AwaitFrame, ConsumeInventoryOptions, DescribeActionForActionText,
    DescribeActionForOpenScreen, MousePress,
};
use screen::{
    action_letters, colors, fuzzy_pixels,inventory_slot_pixels,
    fuzzy_pixels::{action_text_blue, action_text_white},
};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn chop_down_tree_activity() -> ConsumeInventoryOptions {
    ConsumeInventoryOptions {
        timeout: Duration::from_secs(10),
        multi_slot_action: false,
        reset_period: Some(Duration::from_secs(300)),
        inventory_consumption_pixels: vec![inventory_slot_pixels::empty()],
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::tree_bark()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
                action_text: vec![
                    (action_letters::upper_c(), action_text_white()),
                    (action_letters::lower_h(), action_text_white()),
                    (action_letters::lower_o(), action_text_white()),
                    (action_letters::lower_p(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::lower_d(), action_text_white()),
                    (action_letters::lower_o(), action_text_white()),
                    (action_letters::lower_w(), action_text_white()),
                    (action_letters::lower_n(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_t(), action_text_blue()),
                    (action_letters::lower_r(), action_text_blue()),
                    (action_letters::lower_e(), action_text_blue()),
                    (action_letters::lower_e(), action_text_blue()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::forward_slash(), action_text_white()),
                ],
            }),
        ],
    }
}

fn chop_down_oak_activity() -> ConsumeInventoryOptions {
    ConsumeInventoryOptions {
        timeout: Duration::from_secs(20),
        multi_slot_action: true,
        reset_period: Some(Duration::from_secs(300)),
        inventory_consumption_pixels: vec![inventory_slot_pixels::empty()],
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::oak_bark()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(3)),
                action_text: vec![
                    (action_letters::upper_c(), action_text_white()),
                    (action_letters::lower_h(), action_text_white()),
                    (action_letters::lower_o(), action_text_white()),
                    (action_letters::lower_p(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::lower_d(), action_text_white()),
                    (action_letters::lower_o(), action_text_white()),
                    (action_letters::lower_w(), action_text_white()),
                    (action_letters::lower_n(), action_text_white()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::upper_o(), action_text_blue()),
                    (action_letters::lower_a(), action_text_blue()),
                    (action_letters::lower_k(), action_text_blue()),
                    (action_letters::space(), action_text_white()),
                    (action_letters::forward_slash(), action_text_white()),
                ],
            }),
        ],
    }
}

fn get_activity_description(use_oak: bool) -> ConsumeInventoryOptions {
    if use_oak {
        chop_down_oak_activity()
    } else {
        chop_down_tree_activity()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut player = controller::Player::new(config);
    player.consume_inventory(&get_activity_description(/*use_oak=*/ true));

    Ok(())
}
