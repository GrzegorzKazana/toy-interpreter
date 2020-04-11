use super::{ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::Token;

pub fn consume_number_literal(tokens: &[Token]) -> ExpressionParsingResult {
    match tokens.split_first() {
        Option::Some((Token::NumberToken(number_str), rest)) => number_str
            .parse()
            .map(|value| (ExpressionNode::NumberLiteral { value }, rest))
            .ok(),
        _ => Option::None,
    }
}
