mod combined_tokenizer;
mod common;
mod tests;

use combined_tokenizer::CombinedTokenizer;

type SourceRest = String;
type TokenizerResult<T> = Option<(T, SourceRest)>;

#[derive(Debug, PartialEq)]
pub enum Token {
    NumberToken(String),
    OperatorToken(Operator),
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Assignment,
    Comma,
    FunctionKeyword,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub trait Tokenizer<T> {
    fn tokenize(&self, input: &String) -> TokenizerResult<T>;
}

pub struct InputTokenizer {}

impl InputTokenizer {
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        let tokenizer = CombinedTokenizer::new();

        let mut input = String::from(input);
        let mut tokens: Vec<Token> = Vec::new();

        while let Option::Some((matched, rest)) = tokenizer.tokenize(&input) {
            tokens.push(matched);
            input = rest;
        }

        if input.is_empty() {
            Result::Ok(tokens)
        } else {
            Result::Err(format!(
                "Failed to tokenize inputm unexprected input:\n{}",
                input
            ))
        }
    }
}
