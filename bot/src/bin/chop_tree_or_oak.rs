/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::controller;
use screen::{action_letters, colors};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn chop_down_tree_description() -> controller::ActionDescription {
    controller::ActionDescription {
        colors: vec![colors::TREE_BARK],
        timeout: Duration::from_secs(10),
        multi_item_action: false,
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
    }
}

fn chop_down_oak_description() -> controller::ActionDescription {
    controller::ActionDescription {
        colors: vec![colors::OAK_BARK],
        timeout: Duration::from_secs(20),
        multi_item_action: true,
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
    }
}

fn get_action_description(use_oak: bool) -> controller::ActionDescription {
    if use_oak {
        chop_down_oak_description()
    } else {
        chop_down_tree_description()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut inventory_action_descriptions = Vec::<Box<dyn bot::DescribeAction>>::new();
    inventory_action_descriptions.push(Box::new(bot::DescribeActionForOpenScreen {
        expected_pixels: vec![colors::ACTION_BLUE],
        mouse_press: bot::MousePress::Left,
        await_result_time: Duration::from_secs(1),
    }));

    let mut player = controller::Player::new(config);

    player.fill_inventory(&get_action_description(/*use_oak=*/ true));

    Ok(())
}
