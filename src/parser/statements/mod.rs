mod assignment;
mod function_declaration;

use super::ParsingResult;
use crate::tokenizer::Token;
use assignment::consume_assignemnt;
pub use assignment::Assignment;
use function_declaration::consume_function_declaration;
pub use function_declaration::FunctionDeclaration;

#[derive(Debug, PartialEq)]
pub enum StatementNode {
    Assignment(Assignment),
    FunctionDeclaration(FunctionDeclaration),
}

type StatementParsingResult<'a> = ParsingResult<'a, StatementNode>;

pub fn build_statement(tokens: &[Token]) -> StatementParsingResult {
    consume_assignemnt(tokens).or_else(|| consume_function_declaration(tokens))
}
