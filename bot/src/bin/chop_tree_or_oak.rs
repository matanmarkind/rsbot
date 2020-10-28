/// 1. Find tree
/// 2. Move mouse there.
/// 3. Confirm words say tree.
/// 4. Confirm mouse pixel matches tree.
/// 5. Click.
/// 6. How do I know when I have completed? Going to have to work on
///    understanding my inventory.
use bot::{
    controller, AwaitFrame, ConsumeInventoryParams, DescribeActionForActionText,
    DescribeActionForOpenScreen, MousePress,
};
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;

fn chop_down_tree_activity() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        slot_consumption_waittime: Duration::from_secs(10),
        multi_slot_action: false,
        item_to_consume: inventory_slot_pixels::empty(),
        activity_timeout: Duration::from_secs(10 * 60),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::tree_bark()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
                action_text: action_text::chop_down_tree(),
            }),
        ],
    }
}

fn chop_down_oak_activity() -> ConsumeInventoryParams {
    ConsumeInventoryParams {
        slot_consumption_waittime: Duration::from_secs(20),
        multi_slot_action: true,
        item_to_consume: inventory_slot_pixels::empty(),
        activity_timeout: Duration::from_secs(10 * 60),
        actions: vec![
            Box::new(DescribeActionForOpenScreen {
                expected_pixels: vec![fuzzy_pixels::oak_bark()],
                mouse_press: MousePress::None,
                await_action: AwaitFrame::Time(Duration::from_secs(1)),
            }),
            Box::new(DescribeActionForActionText {
                mouse_press: MousePress::Left,
                await_action: AwaitFrame::Time(Duration::from_secs(3)),
                action_text: action_text::chop_down_oak(),
            }),
        ],
    }
}

fn get_activity_description(use_oak: bool) -> ConsumeInventoryParams {
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
