mod tests;

use std::collections::HashMap;

use crate::io::CommandLine;
use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::AssignmentNode;
use crate::parser::Parser;
use crate::tokenizer::InputTokenizer;
use crate::visitor::Visitor;

pub struct Interpreter {
    variables: HashMap<String, isize>,
    command_line: CommandLine,
    tokenizer: InputTokenizer,
    parser: Parser,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            command_line: CommandLine {},
            tokenizer: InputTokenizer {},
            parser: Parser {},
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

    pub fn handle_input(&mut self, input_str: &str) -> Result<String, String> {
        let tokens = self.tokenizer.tokenize(&input_str)?;
        let ast = self.parser.parse(&tokens)?;
        let result = self
            .visit(&ast)
            .ok_or(String::from("Failed to interpret."))?;

        Result::Ok(format!("{}", result))
    }
}

impl Visitor for Interpreter {
    fn visit_var(&self, node: &Variable) -> Option<isize> {
        self.variables.get(&node.identifier).map(|v| v.clone())
    }

    fn visit_fn_call(&self, _node: &FunctionCall) -> Option<isize> {
        Option::None
    }

    fn visit_assignment(&mut self, node: &AssignmentNode) -> Option<isize> {
        let maybe_value = self.visit_expression(&node.expression);

        match maybe_value {
            Option::Some(value) => {
                self.variables.insert(node.identifier.clone(), value);
                Option::Some(value)
            }
            _ => Option::None,
        }
    }
}
