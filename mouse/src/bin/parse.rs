use structopt::StructOpt;

extern crate mouse;
use mouse::parse_lib;

fn main() {
    let config = parse_lib::Config::from_args();
    dbg!(&config);
    let parser = crate::parse_lib::MousePathParser { config };
    parser.parse();
}
