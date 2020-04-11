use crate::tokenizer::Token;

use super::ParsingResult;

#[derive(Debug)]
pub enum ExpressionNode {
    NumericalExpression {
        node_a: Box<ExpressionNode>,
        op: String,
        node_b: Box<ExpressionNode>,
    },
    NumberLiteral {
        value: u32,
    },
    Variable {
        identifier: String,
    },
}

type ExpressionParsingResult<'a> = ParsingResult<'a, ExpressionNode>;

pub fn build_expression(tokens: &[Token]) -> ExpressionParsingResult {
    consume_math_operation(tokens)
        .or_else(|| consume_parenthesis(tokens))
        .or_else(|| consume_number_literal(tokens))
        .or_else(|| consume_variable_identifier(tokens))
}

fn consume_variable_identifier(tokens: &[Token]) -> ExpressionParsingResult {
    let (head, rest) = tokens.split_first()?;

    match head {
        Token::Identifier(identifier) => Option::Some((
            ExpressionNode::Variable {
                identifier: identifier.to_string(),
            },
            rest,
        )),
        _ => Option::None,
    }
}

fn consume_number_literal(tokens: &[Token]) -> ExpressionParsingResult {
    match tokens.split_first() {
        Option::Some((Token::NumberToken(number_str), rest)) => number_str
            .parse()
            .map(|value| (ExpressionNode::NumberLiteral { value }, rest))
            .ok(),
        _ => Option::None,
    }
}

pub fn consume_math_operation(tokens: &[Token]) -> ExpressionParsingResult {
    fn build_math_expression(tokens: &[Token]) -> ExpressionParsingResult {
        consume_parenthesis(tokens)
            .or_else(|| consume_number_literal(tokens))
            .or_else(|| consume_variable_identifier(tokens))
    }

    let (node_a, rest_a) = build_math_expression(tokens)?;
    let (maybe_operator, rest_after_op) = rest_a.split_first()?;

    match maybe_operator {
        Token::OperatorToken(op) => {
            let (node_b, rest_b) = build_expression(rest_after_op)?;
            let result_node = ExpressionNode::NumericalExpression {
                node_a: Box::new(node_a),
                node_b: Box::new(node_b),
                op: op.clone(),
            };

            Option::Some((result_node, rest_b))
        }
        _ => Option::None,
    }
}

pub fn consume_parenthesis(tokens: &[Token]) -> ExpressionParsingResult {
    let (head, rest) = tokens.split_first()?;

    match head {
        Token::LeftParenthesis => {
            let (expression_node, rest_after_expression) = build_expression(rest)?;
            let (maybe_right_paren, rest_after_paren) = rest_after_expression.split_first()?;

            match maybe_right_paren {
                Token::RightParenthesis => Option::Some((expression_node, rest_after_paren)),
                _ => Option::None,
            }
        }
        _ => Option::None,
    }
}
