use crate::parser::Parser;
use crate::runtime::Runtime;
use crate::tokenizer::InputTokenizer;

pub struct Interpreter {
    tokenizer: InputTokenizer,
    parser: Parser,
    runtime: Runtime,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tokenizer: InputTokenizer {},
            parser: Parser {},
            runtime: Runtime::new(),
        }
    }
}

impl Interpreter {
    pub fn handle_input(&mut self, input_str: &str) -> Result<String, String> {
        let tokens = self.tokenizer.tokenize(&input_str)?;
        let ast = self.parser.parse(&tokens)?;
        let result = self.runtime.exec(&ast)?;

        Result::Ok(format!("{}", result))
    }
}
