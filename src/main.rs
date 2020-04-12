mod interpreter;
mod io;
mod parser;
mod tokenizer;
mod utils;
mod visitor;

fn main() {
    interpreter::Interpreter::new().run();
}
