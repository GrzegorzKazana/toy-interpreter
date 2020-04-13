use super::super::expressions::{build_expression, ExpressionNode};
use super::{StatementNode, StatementParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub identifier: String,
    pub expression: ExpressionNode,
}

pub fn consume_assignemnt(tokens: &[Token]) -> StatementParsingResult {
    let maybe_identifier = tokens.get(0)?;
    let maybe_assignment = tokens.get(1)?;
    let tokens_after_assignment = &tokens[2..];

    match (maybe_identifier, maybe_assignment) {
        (Token::Identifier(identifier), Token::Assignment) => {
            let (expression, rest) = build_expression(tokens_after_assignment)?;
            let result_node = StatementNode::Assignment(Assignment {
                identifier: identifier.clone(),
                expression,
            });

            Option::Some((result_node, rest))
        }
        _ => Option::None,
    }
}
