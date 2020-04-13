use crate::io::CommandLine;
use crate::parser::Parser;
use crate::runtime::Runtime;
use crate::tokenizer::InputTokenizer;

pub struct Interpreter {
    command_line: CommandLine,
    tokenizer: InputTokenizer,
    parser: Parser,
    runtime: Runtime,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            command_line: CommandLine {},
            tokenizer: InputTokenizer {},
            parser: Parser {},
            runtime: Runtime::new(),
        }
    }
}

impl Interpreter {
    pub fn run(&mut self) {
        while let Option::Some(input_str) = self.command_line.get_input() {
            match self.handle_input(&input_str) {
                Result::Ok(msg) => self.command_line.print_output(msg),
                Result::Err(msg) => self.command_line.print_output(msg),
            }
        }
    }

    fn handle_input(&mut self, input_str: &str) -> Result<String, String> {
        let tokens = self.tokenizer.tokenize(&input_str)?;
        let ast = self.parser.parse(&tokens)?;
        let result = self.runtime.exec(&ast)?;

        Result::Ok(format!("{}", result))
    }
}
