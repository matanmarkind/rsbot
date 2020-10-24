/// Take a screenshot of the game and draw on the spots that are checked for
/// these letters. Used to add new action_letters.
use screen::{
    action_letters,
    fuzzy_pixels::{action_text_blue, action_text_orange, action_text_white},
};
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
        (action_letters::start(), action_text_white()),
        (action_letters::upper_m(), action_text_white()),
        (action_letters::lower_i(), action_text_white()),
        (action_letters::lower_n(), action_text_white()),
        (action_letters::lower_e(), action_text_white()),
        (action_letters::space(), action_text_white()),
        (action_letters::upper_r(), action_text_blue()),
        (action_letters::lower_o(), action_text_blue()),
        (action_letters::lower_c(), action_text_blue()),
        (action_letters::lower_k(), action_text_blue()),
        (action_letters::lower_s(), action_text_blue()),
        (action_letters::space(), action_text_white()),
        (action_letters::forward_slash(), action_text_white()),
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
