use screen::{
    action_text, fuzzy_pixels, ActionText, Capturer, Frame, FrameHandler, FuzzyPixel,
    InventorySlotPixels, Locations,
};
use std::thread::sleep;
use std::time::Duration;
use userinput::InputBot;
use util::*;

/// This trait is used to define the interface that controls how the bot will
/// behave. In it we pass all of the primitives needed for the bot to interact
/// with and understand the game.
///
/// Actions are meant to be composable so that we can build pieces out of them,
/// and then create a higher level activity that fulfills this trait by calling
/// to other actions.
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

/// Here we implement simple actions, specifically they are non-compound (aka do
/// not call to other Actions).
///
/// These represent specific steps that the player will take without
/// representing a meaningful activity.

/// Make sure the player is in walking, not running, mode.
pub struct EnableWalking {}

/// Make the player walk in a straight line by clicking on the minimap. Because
/// this is timed and we don't measure where we are, we only walk. This is to
/// avoid expecting us to run and then being unable to and being thrown off.
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

impl Action for EnableWalking {
    fn do_action(
        &self,
        inputbot: &mut InputBot,
        framehandler: &mut FrameHandler,
        capturer: &mut Capturer,
    ) -> bool {
        let frame = capturer.frame().unwrap();
        let pos = framehandler.locations.run_icon();
        if !frame.check_loose_pixel(&pos, &fuzzy_pixels::run_icon_on()) {
            // AFAICT the boot doesn't change color, only the surrounding
            // circle, when the mouse hovers over it. So no need to check for
            // that.

            // dbg!("worldmap already open");
            return true;
        }

        inputbot.move_to(&pos);
        inputbot.left_click();
        true
    }
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
        let enable_walking = EnableWalking {};
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
