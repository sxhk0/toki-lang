use std::{env, fs};

use toki_parser::parser;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let first_argument = &arguments[1];

    let script_contents = fs::read_to_string(first_argument).expect(&format!(
        "Failed to read contents of \"{:?}\"",
        first_argument
    ));

    let ast = parser::parse(&script_contents);

    println!("{:?}", ast);
}
