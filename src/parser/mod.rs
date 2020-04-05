mod combined_parser;
mod common;
mod number_parser;
mod operator_parser;

use combined_parser::CombinedParser;

type SourceRest = String;
type ParsingResult<T> = Option<(T, SourceRest)>;

#[derive(Debug)]
pub enum Token {
    NumberToken(String),
    OperatorToken(String),
}

pub trait Parser<T> {
    fn parse(&self, input: &String) -> ParsingResult<T>;
}

pub fn parse(input: &str) -> Result<Vec<Token>, &str> {
    let parser = CombinedParser::new();

    let mut parsing_input = String::from(input);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let res = parser.parse(&parsing_input);
        match res {
            Option::Some((matched, rest)) => {
                tokens.push(matched);
                parsing_input = rest;
            }
            Option::None => {
                break if parsing_input.is_empty() {
                    Result::Ok(tokens)
                } else {
                    Result::Err("Failed to parse input")
                }
            }
        };
    }
}
