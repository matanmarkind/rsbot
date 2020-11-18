use bot::actions::*;
use screen::{action_text, fuzzy_pixels, ActionText, Capturer, Frame, FrameHandler, FuzzyPixel};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use strum_macros::EnumString;
use userinput::InputBot;

#[derive(Debug, Copy, Clone, EnumString)]
pub enum Enemy {
    Cow,
    AlKharidWarrior,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long)]
    pub enemy: Enemy,
}

pub fn enemy_pixels(enemy: Enemy) -> Vec<FuzzyPixel> {
    match enemy {
        Enemy::Cow => vec![
            // Only using white for now since cows near falador have
            // surroundings that cause the other colors to have many false
            // positives so slows us down.
            fuzzy_pixels::cow_white(),
            // fuzzy_pixels::cow_black(),
            // fuzzy_pixels::cow_dark_brown(),
            // fuzzy_pixels::cow_light_brown(),
        ],
        Enemy::AlKharidWarrior => vec![
            fuzzy_pixels::al_kharid_warrior_purple1(),
            fuzzy_pixels::al_kharid_warrior_purple2(),
        ],
    }
}

pub fn get_action_text(enemy: Enemy) -> ActionText {
    match enemy {
        Enemy::Cow => action_text::attack_cow(),
        Enemy::AlKharidWarrior => action_text::attack_al_kharid_warrior(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = Capturer::new();
    let mut inputbot = InputBot::new(config.bot_config.userinput_config.clone());
    let mut framehandler = FrameHandler::new(config.bot_config.screen_config.clone());
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
        /*expected_pixels=*/ enemy_pixels(config.enemy),
        /*action_text=*/ Some(get_action_text(config.enemy)),
        /*mouse_click=*/ MouseClick::Left,
    );

    let await_begin_fighting = Await {
        condition: AwaitCondition::Time,
        timeout: Duration::from_secs(5),
    };

    let await_done_fighting = ExplicitActions {
        actions: vec![
            Box::new(AwaitAny {
                conditions: vec![
                    AwaitCondition::PixelMismatch(
                        framehandler.locations.enemy_healthbar_right(),
                        fuzzy_pixels::enemy_healthbar_red(),
                    ),
                    AwaitCondition::PixelMatch(
                        framehandler.locations.enemy_healthbar_left(),
                        fuzzy_pixels::enemy_healthbar_red(),
                    ),
                ],
                timeout: Duration::from_secs(20),
            }),
            // Wait until the body disappears so we dont click on a dead enemy.
            Box::new(Await {
                condition: AwaitCondition::Time,
                timeout: Duration::from_secs(3),
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

        await_begin_fighting.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !fuzzy_pixels::enemy_healthbar_red().matches(
            &capturer
                .frame()
                .unwrap()
                .get_pixel(&framehandler.locations.enemy_healthbar_right()),
        ) {
            inputbot.pan_left(37.0);
            continue;
        }

        let res = await_done_fighting.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            // break;
        }
    }

    Ok(())
}
