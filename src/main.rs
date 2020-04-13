mod interpreter;
mod io;
mod parser;
mod runtime;
mod tokenizer;
mod utils;
mod visitor;

// fun fib(n) = n ? (n - 1 ? fib(n - 1) + fib(n - 2) : 1) : 0
// fun fac(n) = n ? n * fac(n - 1) : 1

fn main() {
    interpreter::Interpreter::new().run();
}
