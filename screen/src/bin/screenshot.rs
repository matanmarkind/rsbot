// From scrap github repo. Here for my convenience.
use screen::{pixels, Capturer, Frame, FrameHandler, Locations, OwnedFrame, Pixel};
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
    let delta = 3;
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

fn marked_open_screen(cap: &mut Capturer, screenhandler: &FrameHandler) -> OwnedFrame {
    let mut frame = cap.frame().unwrap().to_owned();
    dbg!(screenhandler.is_inventory_open(&frame));

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
        &screenhandler.locations.minimap_plus_top_left(),
        &screenhandler.locations.minimap_plus_dimensions(),
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
    surrounding_box(&mut frame, &screenhandler.locations.worldmap_icon());
    surrounding_box(&mut frame, &screenhandler.locations.compass_icon());
    surrounding_box(
        &mut frame,
        &screenhandler.locations.inventory_icon_background(),
    );

    for (pos, dim) in screenhandler.locations.open_screen_search_boxes() {
        frame.draw_red_box(&pos, &dim);
    }

    frame
}

fn marked_worldmap(cap: &mut Capturer, screenhandler: &FrameHandler) -> OwnedFrame {
    let mut frame = cap.frame().unwrap().to_owned();
    dbg!(screenhandler.is_chatbox_open(&frame));
    dbg!(screenhandler.is_worldmap_open(&frame));

    frame.flip_to_rgb();
    frame.draw_red_box(
        &screenhandler.locations.worldmap_top_left(),
        &screenhandler.locations.worldmap_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.worldmap_map_top_left(),
        &screenhandler.locations.worldmap_map_dimensions(),
    );
    surrounding_box(&mut frame, &screenhandler.locations.worldmap_map_middle());
    for (pos, dim) in screenhandler.locations.worldmap_map_search_boxes() {
        frame.draw_red_box(&pos, &dim);
    }
    // Draw an example of searching within a radial slice of the worldmap.
    {
        let DeltaPosition { dx, dy } = screenhandler.locations.worldmap_map_dimensions();
        let min_radius = 30;
        let worldmap_arc_iter = PositionIteratorCircularSpiral::new(
            screenhandler.locations.worldmap_map_middle(),
            min_radius,
            /*d_radius=*/ std::cmp::min(dx, dy) / 2 - min_radius - 1,
            /*min_angle_degrees=*/ 270.0,
            /*d_angle_degrees=*/ 45.0,
            /*spacing=*/ 2,
        );
        for (i, pos) in worldmap_arc_iter.enumerate() {
            // Have the color change to show the order of the iterator.
            frame.recolor_pixel(
                &pos,
                &Pixel {
                    blue: 0,
                    green: 0,
                    red: std::cmp::min(255, i) as u8,
                },
            );
        }
    }
    {
        let minimap_iter = PositionIteratorCircularSpiral::new(
            /*middle=*/ screenhandler.locations.minimap_middle(),
            /*min_radius=*/ Locations::MINIMAP_RADIUS,
            /*d_radius=*/ 1,
            /*min_angle_degrees=*/ 0.0,
            /*d_angle_degrees=*/ 360.0,
            /*spacing=*/ 2,
        );
        for (i, pos) in minimap_iter.enumerate() {
            // Have the color change to show the order of the iterator.
            frame.recolor_pixel(
                &pos,
                &Pixel {
                    blue: 0,
                    green: 0,
                    red: std::cmp::min(255, i) as u8,
                },
            );
        }
    }
    {
        let adjacent_iter = PositionIteratorCircularSpiral::new(
            /*middle=*/ screenhandler.locations.minimap_middle(),
            /*min_radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
            /*d_radius=*/ 1,
            /*min_angle_degrees=*/ 0.0,
            /*d_angle_degrees=*/ 360.0,
            /*spacing=*/ 1,
        );
        for (i, pos) in adjacent_iter.enumerate() {
            // Have the color change to show the order of the iterator.
            frame.recolor_pixel(
                &pos,
                &Pixel {
                    blue: 0,
                    green: 0,
                    red: std::cmp::min(255, i) as u8,
                },
            );
        }
    }

    frame.draw_red_box(
        &screenhandler.locations.chatbox_outer_top_left(),
        &screenhandler.locations.chatbox_outer_dimensions(),
    );
    frame.draw_red_box(
        &screenhandler.locations.chatbox_inner_top_left(),
        &screenhandler.locations.chatbox_inner_dimensions(),
    );
    surrounding_box(&mut frame, &screenhandler.locations.minimap_middle());
    frame
}

fn marked_inventories(cap: &mut Capturer, screenhandler: &FrameHandler) -> OwnedFrame {
    let mut frame = cap.frame().unwrap().to_owned();
    dbg!(screenhandler.is_bank_open(&frame));
    dbg!(screenhandler.is_bank_quantity_all(&frame));
    dbg!(screenhandler.is_bank_quantity_one(&frame));

    frame.flip_to_rgb();
    for i in 0..Locations::NUM_INVENTORY_SLOTS {
        let slot_top_left = screenhandler.locations.inventory_slot_top_left(i);
        let slot_dimensions = screenhandler.locations.inventory_slot_dimensions();
        frame.draw_red_box(&slot_top_left, &slot_dimensions);

        let past_bottom_right = &slot_top_left + &slot_dimensions;
        let slot_check_spacing = Locations::INVENTORY_SLOT_CHECK_SPACING;
        let first_pos = &slot_top_left + &slot_check_spacing;
        let mut pos = first_pos;
        while pos.y < past_bottom_right.y {
            while pos.x < past_bottom_right.x {
                frame.recolor_pixel(&pos, &pixels::red());
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

    frame.draw_red_box(
        &screenhandler.locations.bank_top_left(),
        &screenhandler.locations.bank_dimensions(),
    );

    for i in 0..Locations::NUM_BANK_SLOTS {
        surrounding_box(&mut frame, &screenhandler.locations.bank_slot_center(i));
    }
    surrounding_box(&mut frame, &screenhandler.locations.bank_quantity_all());
    surrounding_box(&mut frame, &screenhandler.locations.bank_quantity_one());
    frame
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);
    let mut ofpath = config.out_dir.clone();

    let mut capturer = screen::Capturer::new();
    let screenhandler = screen::FrameHandler::new(config.screen_config);

    let frame = marked_open_screen(&mut capturer, &screenhandler);
    ofpath.push_str("screenshot_open_screen.png");
    println!("Saving {}. Open the worldmap and the chatbox...", ofpath);
    frame.save(ofpath.as_str());

    let frame = marked_worldmap(&mut capturer, &screenhandler);
    ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_worldmap.png");
    println!("Saving {}. Open the bank...", ofpath);
    frame.save(ofpath.as_str());

    let frame = marked_inventories(&mut capturer, &screenhandler);
    ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_inventories.png");
    println!("Saving {}...", ofpath);
    frame.save(ofpath.as_str());
}
