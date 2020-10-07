/// Take a screenshot of the game and draw lines to separate the characters in
/// the text that describes an action. This is a test to see if they are regular.
use screen::{letters, ACTION_BLUE, ACTION_WHITE};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a slash (e.g. /path/to/dir/ on linux)"
    )]
    pub out_dir: String,
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();

    let letter_and_matchers = vec![
        (letters::upper_c(), ACTION_WHITE),
        (letters::lower_h(), ACTION_WHITE),
        (letters::lower_o(), ACTION_WHITE),
        (letters::lower_p(), ACTION_WHITE),
        (letters::space(), ACTION_WHITE),
        (letters::lower_d(), ACTION_WHITE),
        (letters::lower_o(), ACTION_WHITE),
        (letters::lower_w(), ACTION_WHITE),
        (letters::lower_n(), ACTION_WHITE),
        (letters::space(), ACTION_WHITE),
        (letters::upper_t(), ACTION_BLUE),
        (letters::lower_r(), ACTION_BLUE),
        (letters::lower_e(), ACTION_BLUE),
        (letters::lower_e(), ACTION_BLUE),
        (letters::space(), ACTION_WHITE),
        (letters::forward_slash(), ACTION_WHITE),
    ];

    // Capture a screenshot, crop it to include just the game window, and flip it to RGB.
    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(screen::check_action_letters(&frame, &letter_and_matchers));

    println!("Saving...");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_action_words.png");
    screen::mark_letters_and_save(&frame, ofpath.as_str(), &letter_and_matchers);
}
