mod tests;

use std::collections::HashMap;
use std::fmt;

use crate::parser::expressions::*;
use crate::parser::statements::*;
use crate::parser::{Node, Program};
use crate::tokenizer::Operator;
use crate::utils::flatten_result;

pub type Context<'a> = Option<&'a HashMap<String, isize>>;
pub type VisitResult = Result<VisitResponse, String>;
pub type VisitExpressionResult = Result<isize, String>;
pub type VisitStatementResult = Result<(), String>;

pub enum VisitResponse {
    Nil,
    Value(isize),
}

impl fmt::Display for VisitResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "()"),
            Self::Value(val) => write!(f, "{}", val),
        }
    }
}

pub trait Visitor {
    fn visit(&mut self, program: &Program) -> VisitResult {
        let result = program
            .body
            .iter()
            .map(|node| match node {
                Node::Statement(statement) => {
                    self.visit_statement(statement).map(|_| VisitResponse::Nil)
                }
                Node::Expression(expression) => self
                    .visit_expression(expression, Option::None)
                    .map(|val| VisitResponse::Value(val)),
            })
            .last()
            .ok_or(String::from("asd"));

        flatten_result(result)
    }

    fn visit_statement(&mut self, node: &StatementNode) -> VisitStatementResult {
        match node {
            StatementNode::Assignment(node) => self.visit_assignment(node),
            StatementNode::FunctionDeclaration(node) => self.visit_fn_declaration(node),
        }
    }

    fn visit_expression(&self, node: &ExpressionNode, context: Context) -> VisitExpressionResult {
        match node {
            ExpressionNode::Variable(node) => self.visit_var(node, context),
            ExpressionNode::NumberLiteral(node) => self.visit_num(node),
            ExpressionNode::FunctionCall(node) => self.visit_fn_call(node, context),
            ExpressionNode::NumericalExpression(node) => self.visit_math_expr(node, context),
            ExpressionNode::Negation(node) => self.visit_signed_expr(node, context),
        }
    }

    fn visit_math_expr(
        &self,
        node: &NumericalExpression,
        context: Context,
    ) -> VisitExpressionResult {
        let val_a = self.visit_expression(&*node.node_a, context)?;
        let val_b = self.visit_expression(&*node.node_b, context)?;

        match node.op {
            Operator::Add => Result::Ok(val_a + val_b),
            Operator::Subtract => Result::Ok(val_a - val_b),
            Operator::Multiply => Result::Ok(val_a * val_b),
            Operator::Divide => {
                if val_b != 0 {
                    Result::Ok(val_a / val_b)
                } else {
                    Result::Err(String::from("Operation forbidden."))
                }
            }
        }
    }

    fn visit_signed_expr(&self, node: &Negation, context: Context) -> VisitExpressionResult {
        self.visit_expression(&*node.expression, context)
            .map(|val| -val)
    }

    fn visit_num(&self, node: &NumberLiteral) -> VisitExpressionResult {
        Result::Ok(node.value)
    }

    fn visit_var(&self, node: &Variable, context: Context) -> VisitExpressionResult;
    fn visit_fn_call(&self, node: &FunctionCall, context: Context) -> VisitExpressionResult;
    fn visit_fn_declaration(&mut self, node: &FunctionDeclaration) -> VisitStatementResult;
    fn visit_assignment(&mut self, node: &Assignment) -> VisitStatementResult;
}
