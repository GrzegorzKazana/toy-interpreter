pub mod expressions;
pub mod statement;
mod tests;

use crate::tokenizer::Token;
use expressions::build_expression;
pub use expressions::ExpressionNode;
use statement::build_statement;
pub use statement::StatementNode;

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

pub fn run(tokens: &[Token]) -> Result<Program, &str> {
    let mut left_to_parse = tokens;
    let mut nodes: Vec<Node> = Vec::new();

    loop {
        let res = build(left_to_parse);
        match res {
            Option::Some((node, rest)) => {
                left_to_parse = rest;
                nodes.push(node);
            }
            Option::None => {
                break if left_to_parse.len() == 0 {
                    Result::Ok(Program { body: nodes })
                } else {
                    println!("{:#?}", nodes);
                    println!("{:#?}", left_to_parse);
                    Result::Err("Failed to fully consume the tokens")
                }
            }
        }
    }
}
