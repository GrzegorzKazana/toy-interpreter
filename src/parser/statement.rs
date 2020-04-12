use crate::tokenizer::Token;

use super::expressions::{build_expression, ExpressionNode};
use super::ParsingResult;

#[derive(Debug, PartialEq)]
pub enum StatementNode {
    AssignmentNode(AssignmentNode),
}

#[derive(Debug, PartialEq)]
pub struct AssignmentNode {
    pub identifier: String,
    pub expression: ExpressionNode,
}

type StatementParsingResult<'a> = ParsingResult<'a, StatementNode>;

pub fn consume_assignemnt(tokens: &[Token]) -> StatementParsingResult {
    if tokens.len() < 3 {
        return Option::None;
    }

    match (&tokens[0], &tokens[1], &tokens[2..]) {
        (Token::Identifier(identifier), Token::Assignment, tokens_after_assignment) => {
            let (expression, rest) = build_expression(tokens_after_assignment)?;
            let result_node = StatementNode::AssignmentNode(AssignmentNode {
                identifier: identifier.clone(),
                expression,
            });

            Option::Some((result_node, rest))
        }
        _ => Option::None,
    }
}

pub fn build_statement(tokens: &[Token]) -> StatementParsingResult {
    consume_assignemnt(tokens)
}
