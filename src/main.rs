mod ast;
mod parser;
mod utils;

use ast::try_build_numerical_expression;
use parser::parse;

fn main() {
    // let input = "(2) + 2";
    let input = "2+3";

    match parse(input) {
        Result::Ok(tokens) => {
            println!("{:#?}", tokens);
            let x = try_build_numerical_expression(tokens);
            println!("Ast:");
            println!("{:#?}", x)
        }
        Result::Err(reason) => println!("{}", reason),
    }
}
