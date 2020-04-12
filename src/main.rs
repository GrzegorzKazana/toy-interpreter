mod interpreter;
mod io;
mod parser;
mod runtime;
mod tokenizer;
mod utils;
mod visitor;

fn main() {
    interpreter::Interpreter::new().run();
}
