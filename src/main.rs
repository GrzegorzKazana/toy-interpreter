mod parser;
mod utils;

use parser::parse;

fn main() {
    let input = "2 + 2";

    match parse(input) {
        Result::Ok(tokens) => println!("{:#?}", tokens),
        Result::Err(reason) => println!("{}", reason),
    }
}
