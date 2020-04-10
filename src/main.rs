mod ast;
mod parser;
mod utils;

use ast::run;
use parser::parse;

fn main() {
    // let input = "(2) + 2";
    // let input = "0+((2+3-1)+4)+1";
    let input = "a = 1 + 1\n\
                 b = 2\n\
                 c = a + b";

    match parse(input) {
        Result::Ok(tokens) => {
            println!("{:#?}", tokens);
            let x = run(&tokens);
            println!("Ast:");
            println!("{:#?}", x)
        }
        Result::Err(reason) => println!("{}", reason),
    }
}
