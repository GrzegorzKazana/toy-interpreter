mod interpreter;
mod io;
mod parser;
mod runtime;
mod tokenizer;
mod utils;
mod visitor;

pub fn main() {
    let command_line = io::CommandLine {};
    let mut interpreter = interpreter::Interpreter::new();

    while let Option::Some(input_str) = command_line.get_input() {
        match input_str.trim() {
            "exit" => break,
            "" => continue,
            _ => match interpreter.handle_input(&input_str) {
                Result::Ok(msg) => command_line.print_output(msg),
                Result::Err(err_msg) => command_line.print_output(err_msg),
            },
        }
    }
}
