use structopt::StructOpt;

pub mod parse;

fn main() {
    let parser = parse::MousePathParser {
        config: parse::Config::from_args(),
    };
    parser.parse();
}
