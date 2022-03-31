mod arg_parse;

use arg_parse::{Config};

fn main() {
    let config: Config = arg_parse::get();
    println!("{:#?}", config);
}
