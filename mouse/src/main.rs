use structopt::StructOpt;

pub mod parse;

use parse::CONFIG;

fn main() {
    let args = parse::Config::from_args();
    *CONFIG.write().unwrap() = Some(args);
    parse::parse();
}
