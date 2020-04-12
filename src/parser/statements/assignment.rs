use super::super::expressions::{build_expression, ExpressionNode};
use super::{StatementNode, StatementParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub identifier: String,
    pub expression: ExpressionNode,
}

pub fn consume_assignemnt(tokens: &[Token]) -> StatementParsingResult {
    if tokens.len() < 3 {
        return Option::None;
    }

    match (&tokens[0], &tokens[1], &tokens[2..]) {
        (Token::Identifier(identifier), Token::Assignment, tokens_after_assignment) => {
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
