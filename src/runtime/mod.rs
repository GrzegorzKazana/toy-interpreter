mod tests;

use std::collections::HashMap;

use crate::parser::expressions::{FunctionCall, Variable};
use crate::parser::statements::{Assignment, FunctionDeclaration};
use crate::parser::Program;
use crate::visitor::{Context, VisitExpressionResult, VisitResult, VisitStatementResult, Visitor};

pub struct Runtime {
    variables: HashMap<String, isize>,
    functions: HashMap<String, FunctionDeclaration>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

impl Runtime {
    pub fn exec(&mut self, program: &Program) -> VisitResult {
        self.visit(program)
    }
}

impl Visitor for Runtime {
    fn visit_var(&self, node: &Variable, context: Context) -> VisitExpressionResult {
        context
            .and_then(|val_map| val_map.get(&node.identifier))
            .or_else(|| self.variables.get(&node.identifier))
            .map(|v| v.clone())
            .ok_or(String::from("Variable not declared"))
    }

    fn visit_fn_call(&self, node: &FunctionCall, context: Context) -> VisitExpressionResult {
        let function = self
            .functions
            .get(&node.identifier)
            .ok_or(String::from("Function not declared"))?;

        if function.arguments.len() != node.arguments.len() {
            return Result::Err(format!(
                "Invalid number of arguments, got {} expected {}",
                node.arguments.len(),
                function.arguments.len()
            ));
        }

        let declarations_w_calls = function.arguments.iter().zip(node.arguments.iter());
        let mut function_call_context: HashMap<String, isize> = match context {
            Option::Some(map) => map.clone(),
            _ => HashMap::new(),
        };

        for (declaration, call) in declarations_w_calls {
            let id = declaration.identifier.clone();
            let val = self.visit_expression(call, context)?;
            function_call_context.insert(id, val);
        }
        let return_value =
            self.visit_expression(&function.expression, Option::Some(&function_call_context))?;

        Result::Ok(return_value)
    }

    fn visit_assignment(&mut self, node: &Assignment) -> VisitStatementResult {
        let value = self.visit_expression(&node.expression, Option::None)?;
        self.variables.insert(node.identifier.clone(), value);
        Result::Ok(())
    }

    fn visit_fn_declaration(&mut self, node: &FunctionDeclaration) -> VisitStatementResult {
        self.functions.insert(node.identifier.clone(), node.clone());
        Result::Ok(())
    }
}
