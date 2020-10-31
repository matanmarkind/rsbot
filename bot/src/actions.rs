use screen::{
    action_text, fuzzy_pixels, ActionText, Capturer, Frame, FrameHandler, FuzzyPixel,
    InventorySlotPixels, Locations,
};
use std::thread::sleep;
use std::time::Duration;
use userinput::InputBot;
use util::*;

fn check_map_pixels(
    frame: &screen::DefaultFrame,
    middle: Position,
    min_radius: i32,
    d_radius: i32,
    arc_of_interest: (f32, f32),
    primary_pixel: FuzzyPixel,
    check_pixels: &[FuzzyPixel],
) -> Option<Position> {
    let map_iter = PositionIteratorCircularSpiral::new(
        middle,
        min_radius,
        d_radius,
        /*min_angle_degrees=*/ arc_of_interest.0,
        /*d_angle_degrees=*/ arc_of_interest.1,
        /*spacing=*/ 2,
    );

    for pos in map_iter {
        if !primary_pixel.matches(&frame.get_pixel(&pos)) {
            continue;
        }

        // Check that the found pixel is in the correct situation.
        let mut all_check_pixels_match = true;
        for check_pixel in check_pixels.iter() {
            let adjacent_iter = PositionIteratorCircularSpiral::new(
                /*middle=*/ pos,
                /*min_radius=*/ 1,
                /*d_radius=*/ Locations::CHECK_ADJACENT_MAP_PIXELS_RADIUS,
                /*min_angle_degrees=*/ 0.0,
                /*d_angle_degrees=*/ 360.0,
                /*spacing=*/ 1,
            );

            let mut found_match = false;
            for adjacent_pos in adjacent_iter {
                if check_pixel.matches(&frame.get_pixel(&adjacent_pos)) {
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                all_check_pixels_match = false;
                break;
            }
        }

        // If all of the check pixels matched, we're good to go.
        if all_check_pixels_match {
            return Some(pos);
        }
    }
    None
}

/// This trait is used to define the interface that controls how the bot will
/// behave. In it we pass all of the primitives needed for the bot to interact
/// with and understand the game.
///
/// Actions are meant to be composable so that we can build pieces out of them,
/// and then create a higher level activity that fulfills this trait by calling
/// to other actions.
///
/// We separate actions colloqially into into modules, solely for organization:
///
/// - basic_action - do not call to any other Action.
/// - compound_action - call to concrete Actions.
/// - abstract_action - call to variable Actions set per use case, making use of
///   dyn Action.
pub trait Action {
    /// Perform an action controlling player, which is an interface to the
    /// screen, keyboard, and mouse.
    ///
    /// Note that calling do_action will invalidate a frame that was retrieved
    /// before this call, which makes logical sense since we expect do_action to
    /// change the state of the game.
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool;
}

pub mod basic_action {
    /// Here we implement basic actions, specifically they are
    /// non-compound (aka basict call to other Actions).
    ///
    /// These represent specific steps that the player will take without
    /// representing a meaningful activity.
    use super::*;

    /// Make sure the player is in walking or running mode.
    pub struct MaybeToggleRunning {
        // If true, will attempt to set the player to run.
        pub try_to_run: bool,
    }

    /// Make sure the worldmap is open/closed.
    pub struct MaybeToggleWorldmap {
        // If true, will make sure the worldmap is open. If false will make sure
        // the worldmap is closed.
        pub worldmap_should_be_open: bool,
    }

    /// Use the minimap to identify a destination and walk towards it.
    pub struct TravelToOnMinimap {
        /// The most identifying pixel for the destination we want to reach. This is
        /// an optimization so that we don't have to check every N pixels at every
        /// position. Since we already enforce finding all 'check_pixels' this
        /// doesn't change the logical performance of this action.
        pub primary_pixel: FuzzyPixel,

        /// Pixels other than those in 'primary_pixel' that are expected to be
        /// found adjacent to our destination. These are less identifying things
        /// like color of the floor, since many pixels will have this which aren't
        /// near the destination. These must all be found in close proximity to the
        /// destination.
        pub check_pixels: Vec<FuzzyPixel>,

        /// Arc of the map to search.
        ///
        /// (min_angle_degrees, arc_angle_degrees).
        /// Recommended to use (0.0, 360.0) unless you have strong reason not to.
        pub arc_of_interest: (f32, f32),
    }

    /// Use the worldmap to identify a destination and walk towards it. This only
    /// moves us 1 click, since it is meant to be used in concert with the minimap
    /// after each step.
    ///
    /// These colors are expected to match those of the worldmap (not there will be
    /// some differences since the minimap changes shading throughout the day).
    ///
    /// We assume the player is always at the center of the worldmap, and when we
    /// find the destination we use the minimap to walk in that direction. We don't
    /// search adjacent to the player so as not ot repeat the minimap search.
    pub struct TravelTowardsOnWorldmap {
        /// See fields in TravelToOnMinimap.
        pub primary_pixel: FuzzyPixel,

        pub check_pixels: Vec<FuzzyPixel>,

        pub arc_of_interest: (f32, f32),
    }

    impl MaybeToggleRunning {
        pub fn run() -> MaybeToggleRunning {
            MaybeToggleRunning { try_to_run: true }
        }
        pub fn walk() -> MaybeToggleRunning {
            MaybeToggleRunning { try_to_run: false }
        }
    }

    impl Action for MaybeToggleRunning {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            println!("MaybeToggleRunning [ try_to_run: {}]", self.try_to_run);
            let mouse_pos = inputbot.mouse_position();
            let minimap_middle = framehandler.locations.minimap_middle();
            let over_minimap = (mouse_pos - minimap_middle).distance() <= Locations::MINIMAP_RADIUS;
            // The minimap is the top right of the screen, so anything higher
            // and to the right of this is assumed to be in the minimap plus
            // region. This may make us a bit oversensitive to moving the mouse
            // in case the game is in the middle of the computer screen and the
            // mouse is on another app, but this may cause surprising hover text
            // anyways, so best to play it safe.
            let minimap_plus_bottom_left = Locations::to_bottom_left(
                framehandler.locations.minimap_plus_top_left(),
                framehandler.locations.minimap_plus_dimensions(),
            );
            let over_minimap_plus = mouse_pos.x >= minimap_plus_bottom_left.x
                && mouse_pos.y <= minimap_plus_bottom_left.y;
            if over_minimap_plus && !over_minimap {
                // The mouse is in the minimap plus region, which can cause
                // highlights and hover text to mess with the boot color. If we
                // are actually over the minimap this won't cause those
                // problems, so no need to move.
                inputbot.move_to(&util::random_position_polar(
                    framehandler.locations.minimap_middle(),
                    Locations::MINIMAP_RADIUS,
                ));
            }

            let frame = capturer.frame().unwrap();
            let pos = framehandler.locations.run_icon();
            let is_run_on = frame.check_loose_pixel(&pos, &fuzzy_pixels::run_icon_on());
            if is_run_on == self.try_to_run {
                return true;
            }

            inputbot.move_to(&util::random_position_polar(pos, 4));
            inputbot.left_click();
            true
        }
    }

    impl MaybeToggleWorldmap {
        pub fn open_worldmap() -> MaybeToggleWorldmap {
            MaybeToggleWorldmap {
                worldmap_should_be_open: true,
            }
        }
        pub fn close_worldmap() -> MaybeToggleWorldmap {
            MaybeToggleWorldmap {
                worldmap_should_be_open: false,
            }
        }
    }

    impl Action for MaybeToggleWorldmap {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            let mouse_pos = inputbot.mouse_position();
            if (mouse_pos - framehandler.locations.worldmap_icon()).distance()
                <= (Locations::WROLDMAP_ICON_RADIUS + 2)
            {
                // The mouse is hovering over the worldmap icon, changing it's
                // color. AFAICT no hover text occludes the worldmap so this
                // should be the only problematic position.
                inputbot.move_to(&util::random_position_polar(
                    framehandler.locations.minimap_middle(),
                    Locations::MINIMAP_RADIUS,
                ));
            }

            let is_worldmap_open = framehandler.is_worldmap_open(&capturer.frame().unwrap());
            if is_worldmap_open == self.worldmap_should_be_open {
                return true;
            }

            inputbot.move_to(&util::random_position_polar(
                framehandler.locations.worldmap_icon(),
                Locations::WROLDMAP_ICON_RADIUS - 3,
            ));
            inputbot.left_click();

            // Move to the minimap center. This should take enough time that the
            // worldmap opens. This also puts us on the minimap which is useful
            // for when are navigating on the worldmap.
            inputbot.move_to(&util::random_position_polar(
                framehandler.locations.minimap_middle(),
                Locations::MINIMAP_RADIUS,
            ));
            let time = std::time::Instant::now();
            while time.elapsed() < Duration::from_secs(3) {
                // Sometimes it takes a long time for the worldmap to open. Wait
                // for this.
                let is_worldmap_open = framehandler.is_worldmap_open(&capturer.frame().unwrap());
                if is_worldmap_open == self.worldmap_should_be_open {
                    return true;
                }
            }
            false
        }
    }

    impl Action for TravelToOnMinimap {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            println!("TravelToOnMinimap");

            // Find the destination on the minimap.
            let pos = match check_map_pixels(
                &capturer.frame().unwrap(),
                framehandler.locations.minimap_middle(),
                /*min_radius=*/ 1,
                /*d_radius=*/ Locations::MINIMAP_RADIUS,
                self.arc_of_interest,
                self.primary_pixel,
                &self.check_pixels,
            ) {
                None => return false, // Failed to find the dst.
                Some(pos) => pos,
            };

            inputbot.move_to(&pos);
            inputbot.left_click();

            // Wait until we are nearby or timeout.
            let time = std::time::Instant::now();
            while time.elapsed() < Duration::from_secs(30) {
                match check_map_pixels(
                    &capturer.frame().unwrap(),
                    framehandler.locations.minimap_middle(),
                    /*min_radius=*/ 1,
                    /*d_radius=*/ Locations::MINIMAP_SMALL_RADIUS,
                    /*arc_of_interest=*/ (0.0, 360.0),
                    self.primary_pixel,
                    &self.check_pixels,
                ) {
                    None => (),
                    Some(_) => return true,
                };

                sleep(Duration::from_millis(100));
            }

            false
        }
    }

    impl Action for TravelTowardsOnWorldmap {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            println!("TravelTowardsOnWorldmap");
            let frame = &capturer.frame().unwrap();

            // Find the destination on the worldmap.
            let DeltaPosition { dx, dy } = framehandler.locations.worldmap_map_dimensions();
            let min_radius = 30;
            let worldmap_pos = match check_map_pixels(
                &frame,
                framehandler.locations.worldmap_map_middle(),
                min_radius,
                /*d_radius=*/ std::cmp::min(dx, dy) / 2 - min_radius - 1,
                self.arc_of_interest,
                self.primary_pixel,
                &self.check_pixels,
            ) {
                None => return false, // Failed to find the dst.
                Some(pos) => pos,
            };

            // Now that we have found the destination on the worldmap, we need to
            // translate this to a location on the minimap to press to walk in that
            // direction.
            let angle_rads =
                (worldmap_pos - framehandler.locations.worldmap_map_middle()).angle_rads();
            let minimap_pos = polar_to_cartesian(
                framehandler.locations.minimap_middle(),
                Locations::MINIMAP_RADIUS - 3,
                angle_rads,
            );
            inputbot.move_to(&minimap_pos);
            inputbot.left_click();

            let running = frame.check_loose_pixel(
                &framehandler.locations.run_icon(),
                &fuzzy_pixels::run_icon_on(),
            );
            sleep(Duration::from_secs(if running { 4 } else { 8 }));
            true
        }
    }
}
pub use basic_action::*;

pub mod compound_action {
    /// Here we implement simple actions. They build off of BasicActions and it is
    /// clear from reading their code exactly what they should do.
    use super::*;

    /// Have the player walk in a straight line, clicking on the minimap to walk.
    /// Since this is timed and we don't measure where we are, we only walk. This is
    /// to avoid expecting us to run and then being unable to and being thrown off.
    ///
    /// We recommend relying on this as little as possible, and preferring TravelTo,
    /// since that will find a destination on the map and is better at correcting
    /// for errors.
    ///
    /// Cannot fail.
    pub struct TravelStraight {
        /// Direction that the player should move in in degrees.
        ///
        /// - 0 = East, right
        /// - 90 = South, down
        /// - 180 = West, left
        /// - 270 = North, up
        pub direction_degrees: f32,

        /// Approximate amount of time the player should walk for if. Note that we
        /// will not enforce stopping so it is possible for the player to keep
        /// walking until the last spot clicked on the minimap after this returns.
        pub travel_time: Duration,
    }

    /// Combines usage of the minimap and worldmap to make the player run/walk to a
    /// destination.
    ///
    /// TODO: Add arc_of_interest.
    ///
    /// TODO: Add opening worldmap.
    pub struct TravelTo {
        pub travel_minimap: TravelToOnMinimap,
        pub travel_worldmap: TravelTowardsOnWorldmap,
        pub timeout: Duration,
    }

    impl TravelStraight {
        /// Where on the minimap to click.
        pub fn get_minimap_pos(&self, framehandler: &FrameHandler) -> Position {
            // Don't go to the edge of the minimap since the worldmap
            // juts in so we wouldn't move.
            let minimap_radius = Locations::MINIMAP_RADIUS - 7;
            // Choose a random location on the minimap, near the edge, in the
            // direction we want to go.
            let minimap_pos = util::random_position(
                &polar_to_cartesian(
                    framehandler.locations.minimap_middle(),
                    minimap_radius,
                    util::degrees_to_radians(self.direction_degrees),
                ),
                &DeltaPosition { dx: 3, dy: 3 },
            );
            minimap_pos
        }
    }

    impl Action for TravelStraight {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            // Make sure that we are walking for the timing to be accurate.
            let enable_walking = MaybeToggleRunning::walk();
            enable_walking.do_action(inputbot, framehandler, capturer);

            // Get the location on the minimap that we will press repeatedly to move
            // in a straight line.
            let minimap_pos = self.get_minimap_pos(framehandler);

            // Continually press on this spot until we are done.
            let time = std::time::Instant::now();
            while time.elapsed() < self.travel_time {
                let wait_time = std::cmp::min(
                    self.travel_time
                        .checked_sub(time.elapsed())
                        .unwrap_or(Duration::from_nanos(1)),
                    Duration::from_secs(8),
                );
                inputbot.move_to(&minimap_pos);
                inputbot.left_click();
                sleep(wait_time);
            }

            true
        }
    }

    impl TravelTo {
        pub fn new(
            primary_pixel: FuzzyPixel,
            check_pixels: Vec<FuzzyPixel>,
            arc_of_interest: (f32, f32),
            timeout: Duration,
        ) -> TravelTo {
            TravelTo {
                travel_minimap: TravelToOnMinimap {
                    primary_pixel,
                    check_pixels: check_pixels.clone(),
                    arc_of_interest,
                },
                travel_worldmap: TravelTowardsOnWorldmap {
                    primary_pixel,
                    check_pixels,
                    arc_of_interest,
                },
                timeout,
            }
        }
    }

    impl Action for TravelTo {
        fn do_action(
            &self,
            inputbot: &mut InputBot,
            framehandler: &mut FrameHandler,
            capturer: &mut Capturer,
        ) -> bool {
            MaybeToggleRunning::run().do_action(inputbot, framehandler, capturer);

            let mut is_worldmap_open = false;
            let time = std::time::Instant::now();
            while time.elapsed() < self.timeout {
                if self
                    .travel_minimap
                    .do_action(inputbot, framehandler, capturer)
                {
                    // We should be at the destination.
                    if is_worldmap_open {
                        MaybeToggleWorldmap::close_worldmap().do_action(
                            inputbot,
                            framehandler,
                            capturer,
                        );
                    }
                    return true;
                }

                // We either did not find the destination on the minimap, or we did
                // and we failed to get to it.

                if !is_worldmap_open {
                    is_worldmap_open = MaybeToggleWorldmap::open_worldmap().do_action(
                        inputbot,
                        framehandler,
                        capturer,
                    );
                }

                if !is_worldmap_open
                    || !self
                        .travel_worldmap
                        .do_action(inputbot, framehandler, capturer)
                {
                    // Either the worldmap failed to open or we couldn't even
                    // find the destination on the worldmap.
                    MaybeToggleWorldmap::close_worldmap().do_action(
                        inputbot,
                        framehandler,
                        capturer,
                    );
                    return false;
                }
            }

            if is_worldmap_open {
                MaybeToggleWorldmap::close_worldmap().do_action(inputbot, framehandler, capturer);
            }
            false
        }
    }
}
pub use compound_action::*;
