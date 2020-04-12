use super::{ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub identifier: String,
}

pub fn consume_variable_identifier(tokens: &[Token]) -> ExpressionParsingResult {
    let (head, rest) = tokens.split_first()?;

    match head {
        Token::Identifier(identifier) => Option::Some((
            ExpressionNode::Variable(Variable {
                identifier: identifier.to_string(),
            }),
            rest,
        )),
        _ => Option::None,
    }
}
