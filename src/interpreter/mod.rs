mod tests;

use std::collections::HashMap;

use crate::io::CommandLine;
use crate::parser;
use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::AssignmentNode;
use crate::parser::Program;
use crate::tokenizer;
use crate::tokenizer::Token;
use crate::visitor::Visitor;

pub struct Interpreter {
    variables: HashMap<String, usize>,
    command_line: CommandLine,
    tokenizer: Box<dyn Fn(&str) -> Result<Vec<Token>, &str>>,
    parser: Box<dyn Fn(&[Token]) -> Result<Program, &str>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            command_line: CommandLine {},
            tokenizer: Box::new(tokenizer::run),
            parser: Box::new(parser::run),
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
        let tokens = (self.tokenizer)(&input_str)?;
        let ast = (self.parser)(&tokens)?;
        let result = self
            .visit(&ast)
            .ok_or(String::from("Failed to interpret."))?;

        Result::Ok(format!("{}", result))
    }
}

impl Visitor for Interpreter {
    fn visit_var(&self, node: &Variable) -> Option<usize> {
        self.variables.get(&node.identifier).map(|v| v.clone())
    }

    fn visit_fn_call(&self, _node: &FunctionCall) -> Option<usize> {
        Option::None
    }

    fn visit_assignment(&mut self, node: &AssignmentNode) -> Option<usize> {
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
