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
    Chicken,
    // DO NOT USE I think that the Al Khradir Warriors caused the ban since we
    // can't tell when the enemy is outside the door, so we clicked on
    // inaccessible enemies.
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
        Enemy::Chicken => vec![
            fuzzy_pixels::chicken_brown(),
            fuzzy_pixels::chicken_beige1(),
            fuzzy_pixels::chicken_beige2(),
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
        Enemy::Chicken => action_text::attack_chicken(),
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

    // TODO: Immediately start waiting for pixel match instead of set time.
    let await_begin_fighting = ExplicitActions {
        actions: vec![
            // Wait until the healthbar of the last enemy has disappeared.
            Box::new(Await {
                condition: AwaitCondition::Time,
                timeout: Duration::from_secs(3),
            }),
            Box::new(AwaitAny {
                conditions: vec![
                    AwaitCondition::PixelMatch(
                        framehandler.locations.enemy_healthbar_right(),
                        fuzzy_pixels::enemy_healthbar_green(),
                    ),
                    AwaitCondition::PixelMatch(
                        framehandler.locations.enemy_healthbar_right(),
                        fuzzy_pixels::enemy_healthbar_red(),
                    ),
                ],
                timeout: Duration::from_secs(5),
            }),
        ],
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
    let mut just_failed_to_start_fight = false;
    while time.elapsed() < std::time::Duration::from_secs(3 * 60 * 60) {
        let res = attack_cow_action.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            inputbot.pan_left(37.0);
            continue;
        }

        // TODO: If we fail to start fighting twice in a row, exit/reset. We may be clicking across the fence.

        if await_begin_fighting.do_action(&mut inputbot, &mut framehandler, &mut capturer) {
            just_failed_to_start_fight = false;
        } else {
            if just_failed_to_start_fight {
                // If we keep failing to start a fight we may be attacking an
                // enemy across a wall or fence which is a giveaway for being a
                // bot.
                println!("Failed to start fights consecutively.");
                break;
            }

            just_failed_to_start_fight = true;
            inputbot.pan_left(90.0);
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
