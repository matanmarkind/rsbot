use bot::actions::*;
use screen::{action_text, fuzzy_pixels, inventory_slot_pixels, Capturer, FrameHandler};
use std::error::Error;
use std::time::Duration;
use structopt::StructOpt;
use strum_macros::EnumString;
use userinput::InputBot;

#[derive(Debug, PartialEq, Copy, Clone, EnumString)]
pub enum Tree {
    Tree,
    Oak,
    Willow,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Config {
    #[structopt(flatten)]
    pub bot_config: bot::Config,

    #[structopt(long)]
    pub tree_type: Tree,
}

fn travel_to_bank(config: &Config) -> ExplicitActions {
    let mut actions = ExplicitActions { actions: vec![] };
    if config.tree_type == Tree::Tree {
        // When cutting regular trees we sometimes travel too far north
        // to see the bank.
        actions.actions.push(Box::new(TravelStraight {
            direction_degrees: 90.0,
            travel_time: Duration::from_secs(7),
        }));
    }
    actions.actions.push(Box::new(TravelTo::new(
        /*primary_pixel=*/
        fuzzy_pixels::map_icon_bank_yellow(),
        /*check_pixels=*/
        vec![
            fuzzy_pixels::map_icon_dark_gray(),
            fuzzy_pixels::map_icon_light_gray(),
        ],
        /*arc_of_interest=*/ (0.0, 360.0),
        /*timeout=*/ Duration::from_secs(60),
        /*try_to_run=*/ false,
    )));

    actions
}

fn deposit_in_bank(_config: &Config) -> DepositEntireInventoryToBank {
    DepositEntireInventoryToBank::new(/*bank_pixels=*/ vec![
        fuzzy_pixels::bank_brown1(),
        fuzzy_pixels::bank_brown2(),
        fuzzy_pixels::bank_brown3(),
    ])
}

fn travel_to_trees(config: &Config) -> ExplicitActions {
    match config.tree_type {
        Tree::Tree => ExplicitActions {
            actions: vec![
                Box::new(PressCompass {}),
                Box::new(TravelStraight {
                    direction_degrees: 230.0,
                    travel_time: Duration::from_secs(5),
                }),
                Box::new(TravelTo::new(
                    /*primary_pixel=*/ fuzzy_pixels::dungeon_icon_blue(),
                    /*check_pixels=*/
                    vec![
                        fuzzy_pixels::map_icon_light_gray(),
                        fuzzy_pixels::dungeon_icon_red(),
                        fuzzy_pixels::black(),
                    ],
                    /*arc_of_interest=*/ (0.0, 360.0),
                    /*timeout=*/ Duration::from_secs(60),
                    /*try_to_run=*/ true,
                )),
            ],
        },
        Tree::Oak => ExplicitActions {
            actions: vec![
                Box::new(PressCompass {}),
                Box::new(TravelStraight {
                    direction_degrees: 300.0,
                    travel_time: Duration::from_secs(10),
                }),
                Box::new(TravelStraight {
                    direction_degrees: 270.0,
                    travel_time: Duration::from_secs(11),
                }),
                Box::new(TravelTo::new(
                    /*primary_pixel=*/ fuzzy_pixels::black(),
                    /*check_pixels=*/
                    vec![
                        fuzzy_pixels::map_icon_light_gray(),
                        fuzzy_pixels::map_icon_dark_gray(),
                        fuzzy_pixels::black(),
                    ],
                    /*arc_of_interest=*/ (0.0, 360.0),
                    /*timeout=*/ Duration::from_secs(60),
                    /*try_to_run=*/ true,
                )),
            ],
        },
        Tree::Willow => ExplicitActions {
            actions: vec![Box::new(TravelTo::new(
                /*primary_pixel=*/ fuzzy_pixels::map_icon_fish_dark_blue(),
                /*check_pixels=*/
                vec![
                    fuzzy_pixels::map_icon_light_gray(),
                    fuzzy_pixels::map_icon_fish_light_blue(),
                    fuzzy_pixels::map_icon_fish_medium_blue(),
                    fuzzy_pixels::map_icon_fish_dark_blue(),
                    fuzzy_pixels::black(),
                ],
                /*arc_of_interest=*/ (0.0, 360.0),
                /*timeout=*/ Duration::from_secs(60),
                /*try_to_run=*/ true,
            ))],
        },
    }
}

fn chop_wood(config: &Config) -> ConsumeInventory {
    match config.tree_type {
        Tree::Tree => ConsumeInventory {
            multi_slot_action: false,
            slot_consumption_waittime: Duration::from_secs(15),
            activity_timeout: Duration::from_secs(10 * 60),
            item_to_consume: inventory_slot_pixels::empty(),
            actions: vec![
                // Press minimap middle to close the chatbox before clicking 1.
                Box::new(OpenScreenAction::new(
                    /*expected_pixels=*/
                    vec![fuzzy_pixels::tree_bark()],
                    /*action_text=*/ Some(action_text::chop_down_tree()),
                    /*mouse_click=*/ MouseClick::Left,
                )),
            ],
        },
        Tree::Oak => ConsumeInventory {
            multi_slot_action: true,
            slot_consumption_waittime: Duration::from_secs(10),
            activity_timeout: Duration::from_secs(10 * 60),
            item_to_consume: inventory_slot_pixels::empty(),
            actions: vec![
                // Press minimap middle to close the chatbox before clicking 1.
                Box::new(OpenScreenAction::new(
                    /*expected_pixels=*/
                    vec![fuzzy_pixels::oak_bark()],
                    /*action_text=*/ Some(action_text::chop_down_oak()),
                    /*mouse_click=*/ MouseClick::Left,
                )),
            ],
        },
        Tree::Willow => ConsumeInventory {
            multi_slot_action: true,
            slot_consumption_waittime: Duration::from_secs(10),
            activity_timeout: Duration::from_secs(10 * 60),
            item_to_consume: inventory_slot_pixels::empty(),
            actions: vec![
                // Press minimap middle to close the chatbox before clicking 1.
                Box::new(OpenScreenAction::new(
                    /*expected_pixels=*/
                    vec![fuzzy_pixels::willow_bark1(), fuzzy_pixels::willow_bark2()],
                    /*action_text=*/ Some(action_text::chop_down_willow()),
                    /*mouse_click=*/ MouseClick::Left,
                )),
            ],
        },
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
    1. We start in the draynor bank with an axe equipped, not in the inventory.
"
    );

    let reset_actions = ExplicitActions::default_reset();
    let travel_to_bank_actions = travel_to_bank(&config);
    let deposit_in_bank_actions = deposit_in_bank(&config);
    let travel_to_trees_actions = travel_to_trees(&config);
    let chop_willow_actions = chop_wood(&config);

    let time = std::time::Instant::now();
    let runtime = config.bot_config.runtime();
    while time.elapsed() < runtime {
        let res = reset_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = travel_to_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            deposit_in_bank_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res =
            travel_to_trees_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
        let res = chop_willow_actions.do_action(&mut inputbot, &mut framehandler, &mut capturer);
        if !res {
            dbg!(res);
            break;
        }
    }

    Ok(())
}
