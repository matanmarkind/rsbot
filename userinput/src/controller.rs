use crate::constants::*;
use crate::types::*;
use device_query::{DeviceQuery, DeviceState};
use inputbot::MouseCursor;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::ops::Bound::Included;
use std::thread::sleep;
use std::time::Duration;
use uinput::event::keyboard::Key;
use util::*;
// Struct used to moe the mouse around on the screen.
//
// The quality of the mover is highly dependent on the set of MousePaths given
// in.
struct MouseMover {
    // Map of mouse paths to be followed. {PathSummary : MousePath}
    mouse_paths: MousePaths,

    // Used to get the mouse's coordinates. If this doesn't work we can move to
    device_state: DeviceState,
}

// Maximum distance we can teleport the mouse when 'cheating' towards to a .
// This should be on the order of a smallish DeltaPosition. Used by the
// controller.
pub const MAX_CHEAT_DISTANCE: i32 = 20;

impl MouseMover {
    pub fn new(paths_fpath: &str) -> MouseMover {
        MouseMover {
            mouse_paths: bincode::deserialize(&std::fs::read(paths_fpath).unwrap()[..]).unwrap(),
            device_state: DeviceState::new(),
        }
    }

    pub fn current_position(&self) -> Position {
        let (x, y) = self.device_state.get_mouse().coords;
        Position { x, y }
    }

    /// Move the mouse to 'dst'.
    ///
    /// The mouse follows recorded paths loaded into 'self.mouse_paths'. Once
    /// the mouse is close enough to 'dst', we can simply move the mouse exactly
    /// there in a single hop.
    ///
    /// Returns a bool saying if the mouse reached 'dst'
    ///
    /// # Examples
    ///
    /// see mouse/src/bin/replay.rs for another complex example.
    ///
    /// ```no_run
    /// use util::Position;
    /// use userinput::InputBot;
    ///
    /// fn main() {
    ///     let inputbot = InputBot::new("/path/to/mousepaths.bincode");
    ///     if inputbot.move_to(&Position{x:100, y:100}) {
    ///         println!("You made it!");
    ///     } else {
    ///         println!("Better luck next time.");
    ///     }
    /// }
    /// ```
    pub fn move_to(&self, dst: &Position) -> bool {
        let mut just_cheated = false;
        loop {
            self.follow_path_to(&dst, /*tolerance=*/ 1);
            let distance = self.distance_from(&dst);
            if distance <= MAX_CHEAT_DISTANCE {
                break;
            } else if self.distance_from(&dst) >= distance {
                // We couldn't find a path to move in the right direction, so
                // cheat a little and try again. If we just cheated, exit to
                // avoid an infinite loop.
                if just_cheated {
                    return &self.current_position() == dst;
                }
                cheat_towards(&(dst - &self.current_position()));
                just_cheated = true;
            } else {
                just_cheated = false;
            }
        }

        // Once we are close, we can move exactly to the location in 1 move.
        // This helps avoid stuttering near 'dst' with a bunch of short paths.
        MouseMover::sleep_between_moves();
        self.move_directly_to(&dst);
        &self.current_position() == dst
    }

    fn sleep_between_moves() {
        let mut rng = thread_rng();
        let duration = Uniform::new(MIN_TIME_BETWEEN_LOCATIONS, MAX_TIME_BETWEEN_LOCATIONS);
        sleep(duration.sample(&mut rng));
    }

    fn move_directly_to(&self, dst: &Position) {
        let position = self.current_position();
        let delta = dst - &position;
        MouseCursor::move_abs(delta.dx, delta.dy);
    }

    fn distance_from(&self, dst: &Position) -> i32 {
        let position = self.current_position();
        let delta = dst - &position;
        delta.distance()
    }

    // MousePaths is indexed via PathSummary, which only uses distance as the
    // key. Create a min_distance_summary and a max_distance_summary to search
    // for a matching MousePath.
    fn boundary_summaries(distance: i32, tolerance: i32) -> (PathSummary, PathSummary) {
        (
            PathSummary {
                distance: std::cmp::max(0, distance - tolerance),
                angle_rads: 0.0,
            },
            PathSummary {
                distance: distance + tolerance,
                angle_rads: 0.0,
            },
        )
    }

    // Find a path to get the mouse closer to 'dst' and follow it.
    fn follow_path_to(&self, dst: &Position, tolerance: i32) {
        assert!(tolerance > 0);
        let position = self.current_position();
        let delta = dst - &position;

        // Get an iterator to the relevant paths.
        let (min_distance, max_distance) =
            MouseMover::boundary_summaries(delta.distance(), tolerance);
        let mut relevant_paths = self
            .mouse_paths
            .range((Included(&min_distance), Included(&max_distance)));
        match relevant_paths.next() {
            // Take the first path since it is the shortest. It is better to
            // move too little and then find another path that keeps us going in
            // the same direction than to overshoot and then go back repeatedly
            // since that seems less natural to me.
            Some((summary, path)) => replay_path(summary, path, &delta),
            None => {
                // No path brings the mouse close enough. Expand the tolerance
                // and try again.
                if delta.distance() < tolerance {
                    // Mouse is already close enough, no need to move.
                    return;
                }
                self.follow_path_to(&dst, tolerance + tolerance);
            }
        }
    }
}

/// Follow 'path' in the direction pointed to by 'net_delta'.
///
/// 'summary' is the summary of 'path', specifically of importance is the angle
/// the path moves in, so that we can rotate it to move towards the destination.
///
/// 'path' is the set of deltas to move, each one represents an actual mouse
/// movement.
///
/// 'net_delta' is the arrow from the current location to the destination. It
/// gives the angle that 'path' must be rotated to.
fn replay_path(summary: &PathSummary, path: &MousePath, net_delta: &DeltaPosition) {
    let rotation_needed = net_delta.angle_rads() - summary.angle_rads;

    for delta in path {
        // Move the value an absolute distance across the screen (ie num
        // pixels).
        let DeltaPosition { dx, dy } = delta.rotate(rotation_needed);
        MouseCursor::move_abs(dx, dy);
        MouseMover::sleep_between_moves();
    }
}

// If the mouse gets stuck and we can't find a path to follow in the direction
// of the destination, manually move the mouse one step. This is particularly
// relevant when on the edge of the screen.
fn cheat_towards(net_delta: &DeltaPosition) {
    let summary = PathSummary {
        distance: MAX_CHEAT_DISTANCE,
        angle_rads: 0.0,
    };
    let path: MousePath = vec![DeltaPosition {
        dx: MAX_CHEAT_DISTANCE,
        dy: 0,
    }];
    replay_path(&summary, &path, net_delta);
}

/// Controller for user friendly input to fake a mouse and keyboard.
/// Logic for how to move the mouse around will be encapsulated in the MouseMover.
pub struct InputBot {
    keyboard: uinput::Device,
    mouse: MouseMover,
}

/// Time to wait between press and release of mouse buttons.
// TODO: consider changing usage from Uniform to Normal distribution.
// TODO: consider different values for key and mouse press.
const MIN_CLICK_WAIT: Duration = Duration::from_millis(100);
const MAX_CLICK_WAIT: Duration = Duration::from_millis(150);

impl InputBot {
    pub fn new(paths_fpath: &str) -> InputBot {
        InputBot {
            keyboard: uinput::default()
                .unwrap()
                .name("keyboard")
                .unwrap()
                .event(uinput::event::Keyboard::All)
                .unwrap()
                .create()
                .unwrap(),
            mouse: MouseMover::new(paths_fpath),
        }
    }

    fn click_key(&mut self, key: Key) {
        let mut rng = rand::thread_rng();
        let duration = Uniform::new(MIN_CLICK_WAIT, MAX_CLICK_WAIT);

        self.keyboard.press(&key).unwrap();
        self.keyboard.synchronize().unwrap();

        sleep(duration.sample(&mut rng));

        self.keyboard.release(&key).unwrap();
        self.keyboard.synchronize().unwrap();
    }

    // It's possible that using a single long press will be a red flag, since I
    // think that holding down a key actually sends lots of short presses.
    // Pressing a releasing causes lots of start and stop, perhaps could try to
    // press and do a super short release.
    fn pan(&mut self, degrees: f32, key: &Key) {
        const FULL_ROTATION_TIME: Duration = Duration::from_millis(3755);

        self.keyboard.press(key).unwrap();
        self.keyboard.synchronize().unwrap();

        sleep(FULL_ROTATION_TIME.mul_f32(degrees / 360.0));

        self.keyboard.release(key).unwrap();
        self.keyboard.synchronize().unwrap();
    }

    // Pressing the mouse buttons is basically stateless, but makes sense to
    // put it here for simplicity.
    fn click_mouse(&self, button: &inputbot::MouseButton) {
        // TODO: Consider moving uniform to normal distribution.
        let mut rng = rand::thread_rng();
        let duration = Uniform::new(MIN_CLICK_WAIT, MAX_CLICK_WAIT);

        button.press();
        sleep(duration.sample(&mut rng));
        button.release();
    }

    /// Number of degrees to pan the screen to the left.
    pub fn pan_left(&mut self, degrees: f32) {
        self.pan(degrees, &Key::A);
    }
    pub fn pan_right(&mut self, degrees: f32) {
        self.pan(degrees, &Key::D);
    }

    /// Mouse interactions.
    pub fn left_click(&self) {
        self.click_mouse(&inputbot::MouseButton::LeftButton);
    }
    pub fn right_click(&self) {
        self.click_mouse(&inputbot::MouseButton::RightButton);
    }
    pub fn move_to(&self, dst: &Position) -> bool {
        self.mouse.move_to(dst)
    }
    /// Moves the mouse close to 'dst'.
    ///
    /// This is used to avoid pixel perfect placement.
    pub fn move_near(&self, dst: &Position) -> bool {
        use std::cmp::max;

        let mut rng = thread_rng();
        self.move_to(&Position {
            x: dst.x + max(0, rng.gen_range(-1, 2)),
            y: dst.y + max(0, rng.gen_range(-1, 2)),
        })
    }
    pub fn mouse_position(&self) -> Position {
        self.mouse.current_position()
    }

    /// Keyboard keys.
    pub fn click_esc(&mut self) {
        self.click_key(Key::Esc);
    }
}
