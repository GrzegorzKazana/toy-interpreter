mod interpreter;
mod parser;
mod runtime;
mod tokenizer;
mod utils;
mod visitor;

use crate::interpreter::Interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InterpreterJs {
    _interpreter: Interpreter,
}

#[wasm_bindgen]
impl InterpreterJs {
    pub fn new() -> InterpreterJs {
        InterpreterJs {
            _interpreter: Interpreter::new(),
        }
    }

    pub fn interpret(&mut self, input_str: &str) -> String {
        match self._interpreter.handle_input(input_str) {
            Result::Ok(output) => output,
            Result::Err(err) => err,
        }
    }
}
