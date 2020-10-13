// From scrap github repo. Here for my convenience.
use screen::{Frame, OwnedFrame, FRAME_PERIOD};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use structopt::StructOpt;
use util::*;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a \
                 slash (e.g. /path/to/dir/ on linux)"
    )]
    pub out_dir: String,

    #[structopt(flatten)]
    pub screen_config: screen::Config,
}

fn surrounding_box(frame: &mut OwnedFrame, center: &Position) {
    let delta = 5;
    frame.draw_red_box(
        &(center
            + &DeltaPosition {
                dx: -delta,
                dy: -delta,
            }),
        &DeltaPosition {
            dx: 2 * delta,
            dy: 2 * delta,
        },
    );
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let screenhandler = screen::Handler::new(config.screen_config);
    // Get Frame.
    // Mark it up and save.
    // Crop it.

    let frame;
    loop {
        // Wait until there's a frame.
        match capturer.frame() {
            Ok(f) => {
                frame = f;
                break;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(FRAME_PERIOD);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        }
    }
    let mut frame = frame.to_owned();
    frame.flip_to_rgb();

    frame.draw_red_box(
        &screenhandler.locations.top_left,
        &screenhandler.locations.dimensions,
    );
    frame.draw_red_box(
        &screenhandler.locations.action_text_top_left(),
        &DeltaPosition { dx: 300, dy: 20 },
    );
    frame.draw_red_box(
        &screenhandler.locations.chatbox_outer_top_left(),
        &screenhandler.locations.chatbox_outer_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.chatbox_inner_top_left(),
        &screenhandler.locations.chatbox_inner_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.minimap_top_left(),
        &screenhandler.locations.minimap_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.inventory_outer_top_left(),
        &screenhandler.locations.inventory_outer_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.inventory_inner_top_left(),
        &screenhandler.locations.inventory_inner_dimensions(),
    );

    surrounding_box(&mut frame, &screenhandler.locations.mid_screen());
    surrounding_box(&mut frame, &screenhandler.locations.all_chat_button());
    surrounding_box(&mut frame, &screenhandler.locations.minimap_middle());
    surrounding_box(&mut frame, &screenhandler.locations.worldmap_icon());
    surrounding_box(&mut frame, &screenhandler.locations.compass_icon());
    surrounding_box(
        &mut frame,
        &screenhandler.locations.inventory_icon_background(),
    );

    for i in 0..screen::Locations::NUM_INVENTORY_SLOTS {
        let slot_top_left = screenhandler.locations.inventory_slot_top_left(i);
        let slot_dimensions = screenhandler.locations.inventory_slot_dimensions();
        frame.draw_red_box(&slot_top_left, &slot_dimensions);

        let past_bottom_right = &slot_top_left + &slot_dimensions;
        let slot_check_spacing = screen::Locations::INVENTORY_SLOT_CHECK_SPACING;
        let first_pos = &slot_top_left + &slot_check_spacing;
        let mut pos = first_pos;
        while pos.y < past_bottom_right.y {
            while pos.x < past_bottom_right.x {
                frame.recolor_pixel(&pos,&screen::colors::PURE_RED);
                pos = Position {
                    x: pos.x + slot_check_spacing.dx,
                    y: pos.y,
                };
            }
            pos = Position {
                x: first_pos.x,
                y: pos.y + slot_check_spacing.dy,
            };
        }
    }

    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot.png");
    println!("Saving {}. Open the worldmap...", ofpath);
    frame.save(ofpath.as_str());

    frame = capturer.frame().unwrap().to_owned();
    frame.flip_to_rgb();
    frame.draw_red_box(
        &screenhandler.locations.worldmap_top_left(),
        &screenhandler.locations.worldmap_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.worldmap_map_top_left(),
        &screenhandler.locations.worldmap_map_dimensions(),
    );

    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_worldmap.png");
    println!("Saving {}...", ofpath);
    frame.save(ofpath.as_str());
}
