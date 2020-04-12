mod tests;

use std::collections::HashMap;

use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::{Assignment, FunctionDeclaration};
use crate::parser::Program;
use crate::visitor::Visitor;

pub struct Runtime {
    variables: HashMap<String, isize>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
        }
    }
}

impl Runtime {
    pub fn exec(&mut self, program: &Program) -> Option<isize> {
        self.visit(program)
    }
}

impl Visitor for Runtime {
    fn visit_var(&self, node: &Variable) -> Option<isize> {
        self.variables.get(&node.identifier).map(|v| v.clone())
    }

    fn visit_fn_call(&self, _node: &FunctionCall) -> Option<isize> {
        Option::None
    }

    fn visit_assignment(&mut self, node: &Assignment) -> Option<isize> {
        let maybe_value = self.visit_expression(&node.expression);

        match maybe_value {
            Option::Some(value) => {
                self.variables.insert(node.identifier.clone(), value);
                Option::Some(value)
            }
            _ => Option::None,
        }
    }

    fn visit_fn_declaration(&mut self, node: &FunctionDeclaration) -> Option<isize> {
        Option::None
    }
}
