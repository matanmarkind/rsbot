use bot::actions::*;
use screen::{action_text, fuzzy_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use userinput::InputBot;

fn main() -> Result<(), Box<dyn Error>> {
    let config = bot::Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.userinput_config);
    let mut framehandler = FrameHandler::new(config.screen_config);
    // Starting with the inventory full of uncooked pizzas is an optimization to
    // avoid putting reset between deposit and withdraw.
    println!(
        "\
Assumes that:
    1. We are in a cow pasture appropriately armed.
    2. We are high enough level not to need healing.
"
    );

    let attack_cow_action = OpenScreenAction::new(
        /*expected_pixels=*/
        vec![
            // Only using white for now since cows near falador have
            // surroundings that cause the other colors to have many false
            // positives so slows us down.
            fuzzy_pixels::cow_white(),
            // fuzzy_pixels::cow_black(),
            // fuzzy_pixels::cow_dark_brown(),
            // fuzzy_pixels::cow_light_brown(),
        ],
        /*action_text=*/ Some(action_text::attack_cow()),
        /*mouse_click=*/ MouseClick::Left,
    );

    let await_done_fighting = ExplicitActions {
        actions: vec![
            Box::new(Await {
                condition: AwaitCondition::Time,
                timeout: Duration::from_secs(10),
            }),
            Box::new(Await {
                condition: AwaitCondition::PixelMismatch(
                    framehandler.locations.enemy_healthbar_right(),
                    fuzzy_pixels::enemy_healthbar_red(),
                ),
                timeout: Duration::from_secs(20),
            }),
        ],
    };

    ExplicitActions::default_reset().do_action(&mut inputbot, &mut framehandler, &mut capturer);
    // Run so that we don't waste too much time getting to the cow.
    MaybeToggleRunning::run().do_action(&mut inputbot, &mut framehandler, &mut capturer);
    let time = std::time::Instant::now();
    while time.elapsed() < std::time::Duration::from_secs(3 * 60 * 60) {
        let res = attack_cow_action.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            inputbot.pan_left(37.0);
            continue;
        }

        let res = await_done_fighting.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
