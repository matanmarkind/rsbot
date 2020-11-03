/// Take a screenshot of the game and draw on the spots that are checked for
/// these letters. Used to add new action_letters.
use screen::action_text;
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

    let text = &action_text::open_door();

    println!("Capturing, cropping, flipping, drawing...");
    let frame = capturer.frame().unwrap();
    dbg!(screenhandler.check_action_text(&frame, &text));

    let mut ofpath = config.out_dir.clone();
    ofpath.push_str("screenshot_action_words.png");
    println!("Saving {} ...", ofpath);
    screenhandler
        .mark_letters_and_save(&frame, ofpath.as_str(), &text)
        .join()
        .expect("Error waiting for image to save");
}
