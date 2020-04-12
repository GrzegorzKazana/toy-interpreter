mod tests;

use crate::parser::expressions::*;
use crate::parser::statements::*;
use crate::parser::{Node, Program};
use crate::tokenizer::Operator;

pub trait Visitor {
    fn visit(&mut self, program: &Program) -> Option<usize> {
        let results = program.body.iter().map(|node| match node {
            Node::Statement(statement) => self.visit_statement(statement),
            Node::Expression(expression) => self.visit_expression(expression),
        });

        results.last().flatten()
    }

    fn visit_statement(&mut self, node: &StatementNode) -> Option<usize> {
        match node {
            StatementNode::AssignmentNode(node) => self.visit_assignment(node),
        }
    }

    fn visit_expression(&self, node: &ExpressionNode) -> Option<usize> {
        match node {
            ExpressionNode::Variable(node) => self.visit_var(node),
            ExpressionNode::NumberLiteral(node) => self.visit_num(node),
            ExpressionNode::FunctionCall(node) => self.visit_fn_call(node),
            ExpressionNode::NumericalExpression(node) => self.visit_math_expr(node),
        }
    }

    fn visit_math_expr(&self, node: &NumericalExpression) -> Option<usize> {
        let val_a = self.visit_expression(&*node.node_a)?;
        let val_b = self.visit_expression(&*node.node_b)?;

        match node.op {
            Operator::Add => Option::Some(val_a + val_b),
            Operator::Subtract => Option::Some(val_a - val_b),
            Operator::Multiply => Option::Some(val_a * val_b),
            Operator::Divide => {
                if val_b != 0 {
                    Option::Some(val_a / val_b)
                } else {
                    Option::None
                }
            }
        }
    }

    fn visit_num(&self, node: &NumberLiteral) -> Option<usize> {
        Option::Some(node.value)
    }

    fn visit_var(&self, node: &Variable) -> Option<usize>;
    fn visit_fn_call(&self, node: &FunctionCall) -> Option<usize>;
    fn visit_assignment(&mut self, node: &AssignmentNode) -> Option<usize>;
}
