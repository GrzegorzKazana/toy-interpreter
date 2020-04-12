mod tests;

use std::collections::HashMap;

use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::{Assignment, FunctionDeclaration};
use crate::parser::Program;
use crate::visitor::Visitor;

pub struct Runtime {
    variables: HashMap<String, isize>,
    functions: HashMap<String, FunctionDeclaration>,
    function_call_context: HashMap<String, isize>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            functions: HashMap::new(),
            function_call_context: HashMap::new(),
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
        self.function_call_context
            .get(&node.identifier)
            .or_else(|| self.variables.get(&node.identifier))
            .map(|v| v.clone())
    }

    fn visit_fn_call(&mut self, node: &FunctionCall) -> Option<isize> {
        let function = self.functions.get(&node.identifier)?.clone();

        if function.arguments.len() != node.arguments.len() {
            return Option::None;
        }

        let declarations_w_calls = function.arguments.iter().zip(node.arguments.iter());

        for (declaration, call) in declarations_w_calls {
            let id = declaration.identifier.clone();
            let val = self.visit_expression(call)?;
            self.function_call_context.insert(id, val);
        }
        let return_value = self.visit_expression(&function.expression)?;

        self.function_call_context.clear();

        Option::Some(return_value)
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
        self.functions.insert(node.identifier.clone(), node.clone());
        Option::Some(42)
    }
}
