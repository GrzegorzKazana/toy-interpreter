pub mod expressions;
pub mod statements;
mod tests;

use crate::tokenizer::Token;
use expressions::build_expression;
pub use expressions::ExpressionNode;
use statements::build_statement;
pub use statements::StatementNode;

#[derive(Debug, PartialEq)]
pub enum Node {
    Expression(ExpressionNode),
    Statement(StatementNode),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Node>,
}

pub type ParsingResult<'a, T> = Option<(T, &'a [Token])>;

pub fn build(tokens: &[Token]) -> ParsingResult<Node> {
    build_statement(tokens)
        .map(|(node, rest)| (Node::Statement(node), rest))
        .or_else(|| build_expression(tokens).map(|(node, rest)| (Node::Expression(node), rest)))
}

pub struct Parser {}

impl Parser {
    pub fn parse(&self, tokens: &[Token]) -> Result<Program, String> {
        let mut left_to_parse = tokens;
        let mut nodes: Vec<Node> = Vec::new();

        while let Option::Some((node, rest)) = build(left_to_parse) {
            left_to_parse = rest;
            nodes.push(node);
        }

        if left_to_parse.len() == 0 {
            Result::Ok(Program { body: nodes })
        } else {
            Result::Err(format!(
                "Failed to fully consume the tokens:\n{:#?}",
                left_to_parse
            ))
        }
    }
}
