use crate::parser::expressions::*;
use crate::parser::statement::*;
use crate::parser::{Node, Program};

pub struct Visitor {}

impl Visitor {
    fn visit(&self, program: &Program) {
        program.body.iter().for_each(|node| match node {
            Node::Statement(statement) => self.visit_statement(statement),
            Node::Expression(expression) => {
                self.visit_expression(expression);
            }
        })
    }

    fn visit_statement(&self, node: &StatementNode) {
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
        Option::None
    }

    fn visit_num(&self, node: &NumberLiteral) -> Option<usize> {
        Option::None
    }

    fn visit_var(&self, node: &Variable) -> Option<usize> {
        Option::None
    }

    fn visit_fn_call(&self, node: &FunctionCall) -> Option<usize> {
        Option::None
    }

    fn visit_assignment(&self, node: &AssignmentNode) {}
}
