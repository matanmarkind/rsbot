/// Take a screenshot of the game and draw lines to separate the characters in
/// the text that describes an action. This is a test to see if they are regular.
use screen::{action_letters, colors};
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
        (action_letters::start(), colors::ACTION_WHITE),
        (action_letters::upper_s(), colors::ACTION_WHITE),
        (action_letters::lower_m(), colors::ACTION_WHITE),
        (action_letters::lower_a(), colors::ACTION_WHITE),
        (action_letters::lower_l(), colors::ACTION_WHITE),
        (action_letters::lower_l(), colors::ACTION_WHITE),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::upper_n(), colors::ACTION_WHITE),
        (action_letters::lower_e(), colors::ACTION_WHITE),
        (action_letters::lower_t(), colors::ACTION_WHITE),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::upper_f(), colors::ACTION_YELLOW),
        (action_letters::lower_i(), colors::ACTION_YELLOW),
        (action_letters::lower_s(), colors::ACTION_YELLOW),
        (action_letters::lower_h(), colors::ACTION_YELLOW),
        (action_letters::lower_i(), colors::ACTION_YELLOW),
        (action_letters::lower_n(), colors::ACTION_YELLOW),
        (action_letters::lower_g(), colors::ACTION_YELLOW),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::lower_s(), colors::ACTION_YELLOW),
        (action_letters::lower_p(), colors::ACTION_YELLOW),
        (action_letters::lower_o(), colors::ACTION_YELLOW),
        (action_letters::lower_t(), colors::ACTION_YELLOW),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::forward_slash(), colors::ACTION_WHITE),
    ];

    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(screen::action_letters::check_action_letters(
        &frame,
        &letter_and_matchers
    ));

    println!("Saving...");
    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_action_words.png");
    action_letters::mark_letters_and_save(&frame, ofpath.as_str(), &letter_and_matchers)
        .join()
        .expect("Error waiting for image to save");
}
