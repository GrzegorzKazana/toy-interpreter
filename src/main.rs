mod parser;
mod tokenizer;
mod utils;

fn main() {
    // let input = "(2) + 2";
    // let input = "0+((2+3-1)+4)+1";
    let input = "a = 1 + 1\n\
                 b = 2\n\
                 c = a + b";

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
