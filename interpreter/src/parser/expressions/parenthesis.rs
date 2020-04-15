use super::{build_expression, ExpressionParsingResult};
use crate::tokenizer::Token;

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
