mod common;
mod tests;

use common as C;

type SourceRest = String;
type TokenizerResult = Option<(Token, SourceRest)>;

#[derive(Debug, PartialEq)]
pub enum Token {
    NumberToken(String),
    OperatorToken(Operator),
    Identifier(String),
    LeftParenthesis,
    RightParenthesis,
    Assignment,
    Comma,
    FunctionKeyword,
    QuestionMark,
    Colon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct InputTokenizer {}

impl InputTokenizer {
    fn get_token(&self, input: &str) -> TokenizerResult {
        C::tokenize_fun_keyword(input)
            .or_else(|| C::tokenize_number(input))
            .or_else(|| C::tokenize_identifier(input))
            .or_else(|| C::tokenize_assignment(input))
            .or_else(|| C::tokenize_comma(input))
            .or_else(|| C::tokenize_question_mark(input))
            .or_else(|| C::tokenize_colon(input))
            .or_else(|| C::tokenize_left_parens(input))
            .or_else(|| C::tokenize_right_parens(input))
            .or_else(|| C::tokenize_addition(input))
            .or_else(|| C::tokenize_subtraction(input))
            .or_else(|| C::tokenize_multiplication(input))
            .or_else(|| C::tokenize_division(input))
    }

    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        let mut current_input = String::from(input);
        let mut tokens: Vec<Token> = Vec::new();

        while let Option::Some((matched, rest)) = self.get_token(&current_input) {
            tokens.push(matched);
            current_input = rest;
        }

        if current_input.is_empty() {
            Result::Ok(tokens)
        } else {
            Result::Err(format!("Failed to tokenize unexpected input:\n{}", input))
        }
    }
}
