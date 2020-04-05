use crate::parser::Token;
use crate::utils::{option_seq2, option_seq3};

enum Node {
    NumericalExpression(NumericalExpressionNode),
    NumberLiteral(NumberLiteralNode),
}

#[derive(Debug)]
pub struct NumericalExpressionNode {
    node_a: NumberLiteralNode,
    node_b: NumberLiteralNode,
    op: String,
}

#[derive(Debug)]
struct NumberLiteralNode {
    value: u32,
}

fn try_build_number_literal(tokens: Vec<Token>) -> Option<NumberLiteralNode> {
    match tokens.first() {
        Option::Some(Token::NumberToken(number_str)) => number_str
            .parse()
            .map(|value| NumberLiteralNode { value })
            .ok()
            .filter(|_| tokens.len() == 1),
        _ => Option::None,
    }
}

fn try_build_number_literal_(token: &Token) -> Option<NumberLiteralNode> {
    match token {
        Token::NumberToken(number_str) => number_str
            .parse()
            .map(|value| NumberLiteralNode { value })
            .ok(),
        _ => Option::None,
    }
}

pub fn try_build_numerical_expression(tokens: Vec<Token>) -> Option<NumericalExpressionNode> {
    let maybe_body = (tokens.get(0), tokens.get(1), tokens.get(2));

    match option_seq3(maybe_body) {
        Option::Some((token_a, Token::OperatorToken(op), token_b)) => option_seq2((
            try_build_number_literal_(token_a),
            try_build_number_literal_(token_b),
        ))
        .filter(|_| tokens.len() == 3)
        .map(|(node_a, node_b)| NumericalExpressionNode {
            node_a,
            node_b,
            op: op.clone(),
        }),
        _ => Option::None,
    }
}
