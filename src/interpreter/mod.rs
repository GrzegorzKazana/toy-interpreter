mod tests;

use std::collections::HashMap;

use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::AssignmentNode;
use crate::visitor::Visitor;

pub struct Interpreter {
    variables: HashMap<String, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }
}

impl Visitor for Interpreter {
    fn visit_var(&self, node: &Variable) -> Option<usize> {
        self.variables.get(&node.identifier).map(|v| v.clone())
    }

    fn visit_fn_call(&self, node: &FunctionCall) -> Option<usize> {
        Option::None
    }

    fn visit_assignment(&mut self, node: &AssignmentNode) {
        let maybe_value = self.visit_expression(&node.expression);

        if let Option::Some(value) = maybe_value {
            self.variables.insert(node.identifier.clone(), value);
        }
    }
}
