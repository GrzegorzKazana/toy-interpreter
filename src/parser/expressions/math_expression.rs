use super::{build_simple_math_expression, ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::{Operator, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct NumericalExpression {
    pub node_a: Box<ExpressionNode>,
    pub op: Operator,
    pub node_b: Box<ExpressionNode>,
}

pub fn consume_math_expression(tokens: &[Token], minimum_prio: usize) -> ExpressionParsingResult {
    let (mut root, mut rest) = build_simple_math_expression(tokens)?;

    while let Option::Some((Token::OperatorToken(op), operator_rest)) = rest.split_first() {
        let current_prio = operator_to_priority(op);
        if current_prio < minimum_prio {
            break;
        }

        let (next_node, next_rest) = consume_math_expression(operator_rest, current_prio + 1)?;

        rest = next_rest;
        root = ExpressionNode::NumericalExpression(NumericalExpression {
            node_a: Box::new(root),
            node_b: Box::new(next_node),
            op: op.clone(),
        });
    }

    Option::Some((root, rest))
}

fn operator_to_priority(op: &Operator) -> usize {
    match op {
        Operator::Add | Operator::Subtract => 1,
        Operator::Multiply | Operator::Divide => 2,
    }
}
