use super::{build_simple_math_expression, ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::{Operator, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct NumericalExpression {
    pub node_a: Box<ExpressionNode>,
    pub op: Operator,
    pub node_b: Box<ExpressionNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Negation {
    pub expression: Box<ExpressionNode>,
}

pub fn consume_math_expression(tokens: &[Token]) -> ExpressionParsingResult {
    consume_math_expression_(tokens, 1)
}

fn consume_math_expression_(tokens: &[Token], minimum_prio: usize) -> ExpressionParsingResult {
    let (mut root, mut rest) = consume_signed_expression(tokens)?;

    while let Option::Some((Token::OperatorToken(op), operator_rest)) = rest.split_first() {
        let current_prio = operator_to_priority(op);
        if current_prio < minimum_prio {
            break;
        }

        let (next_node, next_rest) = consume_math_expression_(operator_rest, current_prio + 1)?;

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

pub fn consume_signed_expression(tokens: &[Token]) -> ExpressionParsingResult {
    let (head, sign_rest) = tokens.split_first()?;

    match head {
        Token::OperatorToken(Operator::Subtract) => {
            build_simple_math_expression(sign_rest).map(|(expr, rest)| {
                (
                    ExpressionNode::Negation(Negation {
                        expression: Box::new(expr),
                    }),
                    rest,
                )
            })
        }
        Token::OperatorToken(Operator::Add) => build_simple_math_expression(sign_rest),
        _ => build_simple_math_expression(tokens),
    }
}
