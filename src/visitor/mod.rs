mod tests;

use crate::parser::expressions::*;
use crate::parser::statements::*;
use crate::parser::{Node, Program};
use crate::tokenizer::Operator;

pub trait Visitor {
    fn visit(&mut self, program: &Program) -> Option<isize> {
        let results = program.body.iter().map(|node| match node {
            Node::Statement(statement) => self.visit_statement(statement),
            Node::Expression(expression) => self.visit_expression(expression),
        });

        results.last().flatten()
    }

    fn visit_statement(&mut self, node: &StatementNode) -> Option<isize> {
        match node {
            StatementNode::Assignment(node) => self.visit_assignment(node),
            StatementNode::FunctionDeclaration(node) => self.visit_fn_declaration(node),
        }
    }

    fn visit_expression(&self, node: &ExpressionNode) -> Option<isize> {
        match node {
            ExpressionNode::Variable(node) => self.visit_var(node),
            ExpressionNode::NumberLiteral(node) => self.visit_num(node),
            ExpressionNode::FunctionCall(node) => self.visit_fn_call(node),
            ExpressionNode::NumericalExpression(node) => self.visit_math_expr(node),
        }
    }

    fn visit_math_expr(&self, node: &NumericalExpression) -> Option<isize> {
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

    fn visit_num(&self, node: &NumberLiteral) -> Option<isize> {
        Option::Some(node.value)
    }

    fn visit_var(&self, node: &Variable) -> Option<isize>;
    fn visit_fn_declaration(&mut self, node: &FunctionDeclaration) -> Option<isize>;
    fn visit_fn_call(&self, node: &FunctionCall) -> Option<isize>;
    fn visit_assignment(&mut self, node: &Assignment) -> Option<isize>;
}
