/// Take a screenshot of the game and draw on the spots that are checked for
/// these letters. Used to add new action_letters.
use screen::{action_letters, colors};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(
        long,
        about = "Path to directory to save screenshots to. Should end with a slash (e.g. /path/to/dir/ on linux)"
    )]
    pub out_dir: String,

    #[structopt(flatten)]
    pub screen_config: screen::Config,
}

fn main() {
    let config = Config::from_args();
    dbg!(&config);

    let mut capturer = screen::Capturer::new();
    let screenhandler = screen::FrameHandler::new(config.screen_config);

    let letter_and_matchers = vec![
        (action_letters::start(), colors::ACTION_WHITE),
        (action_letters::upper_b(), colors::ACTION_WHITE),
        (action_letters::lower_a(), colors::ACTION_WHITE),
        (action_letters::lower_n(), colors::ACTION_WHITE),
        (action_letters::lower_k(), colors::ACTION_WHITE),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::upper_b(), colors::ACTION_BLUE),
        (action_letters::lower_a(), colors::ACTION_BLUE),
        (action_letters::lower_n(), colors::ACTION_BLUE),
        (action_letters::lower_k(), colors::ACTION_BLUE),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::lower_b(), colors::ACTION_BLUE),
        (action_letters::lower_o(), colors::ACTION_BLUE),
        (action_letters::lower_o(), colors::ACTION_BLUE),
        (action_letters::lower_t(), colors::ACTION_BLUE),
        (action_letters::lower_h(), colors::ACTION_BLUE),
        (action_letters::space(), colors::ACTION_WHITE),
        (action_letters::forward_slash(), colors::ACTION_WHITE),
    ];

    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(screenhandler.check_action_letters(&frame, &letter_and_matchers));

    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_action_words.png");
    println!("Saving {} ...", ofpath);
    screenhandler
        .mark_letters_and_save(&frame, ofpath.as_str(), &letter_and_matchers)
        .join()
        .expect("Error waiting for image to save");
}
