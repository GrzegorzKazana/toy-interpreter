use super::{ExpressionNode, ExpressionParsingResult, ParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub identifier: String,
}

pub fn consume_variable_identifier(tokens: &[Token]) -> ExpressionParsingResult {
    consume_variable_identifier_helper(tokens).map(|(v, rest)| (ExpressionNode::Variable(v), rest))
}

pub fn consume_variable_identifier_helper(tokens: &[Token]) -> ParsingResult<Variable> {
    let (head, rest) = tokens.split_first()?;

    match head {
        Token::Identifier(identifier) => Option::Some((
            Variable {
                identifier: identifier.to_string(),
            },
            rest,
        )),
        _ => Option::None,
    }
}
