/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::{
    controller, Activity, DescribeActionForActionText, DescribeActionForOpenScreen, FillInventory,
    MousePress,
};
use screen::{action_letters, colors};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn chop_down_tree_activity() -> FillInventory {
    FillInventory {
        timeout: Duration::from_secs(10),
        multi_item_action: false,
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![colors::TREE_BARK],
                mouse_press: MousePress::None,
                await_result_time: Duration::from_nanos(1),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_result_time: Duration::from_nanos(1),
                action_text: vec![
                    (action_letters::upper_c(), colors::ACTION_WHITE),
                    (action_letters::lower_h(), colors::ACTION_WHITE),
                    (action_letters::lower_o(), colors::ACTION_WHITE),
                    (action_letters::lower_p(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::lower_d(), colors::ACTION_WHITE),
                    (action_letters::lower_o(), colors::ACTION_WHITE),
                    (action_letters::lower_w(), colors::ACTION_WHITE),
                    (action_letters::lower_n(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::upper_t(), colors::ACTION_BLUE),
                    (action_letters::lower_r(), colors::ACTION_BLUE),
                    (action_letters::lower_e(), colors::ACTION_BLUE),
                    (action_letters::lower_e(), colors::ACTION_BLUE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::forward_slash(), colors::ACTION_WHITE),
                ],
            }),
        ],
    }
}

fn chop_down_oak_activity() -> FillInventory {
    FillInventory {
        timeout: Duration::from_secs(20),
        multi_item_action: true,
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![colors::OAK_BARK],
                mouse_press: MousePress::None,
                await_result_time: Duration::from_nanos(1),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_result_time: Duration::from_nanos(1),
                action_text: vec![
                    (action_letters::upper_c(), colors::ACTION_WHITE),
                    (action_letters::lower_h(), colors::ACTION_WHITE),
                    (action_letters::lower_o(), colors::ACTION_WHITE),
                    (action_letters::lower_p(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::lower_d(), colors::ACTION_WHITE),
                    (action_letters::lower_o(), colors::ACTION_WHITE),
                    (action_letters::lower_w(), colors::ACTION_WHITE),
                    (action_letters::lower_n(), colors::ACTION_WHITE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::upper_o(), colors::ACTION_BLUE),
                    (action_letters::lower_a(), colors::ACTION_BLUE),
                    (action_letters::lower_k(), colors::ACTION_BLUE),
                    (action_letters::space(), colors::ACTION_WHITE),
                    (action_letters::forward_slash(), colors::ACTION_WHITE),
                ],
            }),
        ],
    }
}

fn get_activity_description(use_oak: bool) -> FillInventory {
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

    get_activity_description(/*use_oak=*/ true).do_activity(&mut player);

    Ok(())
}
