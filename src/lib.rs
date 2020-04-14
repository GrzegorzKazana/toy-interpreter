mod interpreter;
mod parser;
mod runtime;
mod tokenizer;
mod utils;
mod visitor;

use crate::interpreter::Interpreter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JsInterpreter {
    _interpreter: Interpreter,
}

#[wasm_bindgen]
impl JsInterpreter {
    pub fn new() -> JsInterpreter {
        JsInterpreter {
            _interpreter: Interpreter::new(),
        }
    }

    pub fn interpret(&mut self, input_str: &str) -> Result<String, JsValue> {
        self._interpreter
            .handle_input(input_str)
            .or_else(|err_msg| Result::Err(JsValue::from_str(&err_msg)))
    }
}
