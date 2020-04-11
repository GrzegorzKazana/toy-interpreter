mod combined_tokenizer;
mod common;

use combined_tokenizer::CombinedTokenizer;

type SourceRest = String;
type TokenizerResult<T> = Option<(T, SourceRest)>;

#[derive(Debug)]
pub enum Token {
    NumberToken(String),
    OperatorToken(String),
    LeftParenthesis,
    RightParenthesis,
    Identifier(String),
    Assignment,
    Comma,
}

pub trait Tokenizer<T> {
    fn tokenize(&self, input: &String) -> TokenizerResult<T>;
}

pub fn run(input: &str) -> Result<Vec<Token>, &str> {
    let tokenizer = CombinedTokenizer::new();

    let mut input = String::from(input);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let res = tokenizer.tokenize(&input);
        match res {
            Option::Some((matched, rest)) => {
                tokens.push(matched);
                input = rest;
            }
            Option::None => {
                break if input.is_empty() {
                    Result::Ok(tokens)
                } else {
                    Result::Err("Failed to tokenize input")
                }
            }
        };
    }
}
