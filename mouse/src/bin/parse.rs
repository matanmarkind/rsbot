use structopt::StructOpt;

extern crate mouse;
use mouse::parse_lib;

fn main() {
    let parser = crate::parse_lib::MousePathParser {
        config: parse_lib::Config::from_args(),
    };
    parser.parse();
}
