mod parser;
mod tokenizer;
mod utils;

fn main() {
    // let input = "(2) + 2";
    let input = "1+((2+3)+4)";
    // let input = "a = 1 + 1
    //              b = 2
    //              c = a + b";

    match tokenizer::run(input) {
        Result::Ok(tokens) => {
            println!("{:#?}", tokens);
            let x = parser::run(&tokens);
            println!("Ast:");
            println!("{:#?}", x)
        }
        Result::Err(reason) => println!("{}", reason),
    }
}
