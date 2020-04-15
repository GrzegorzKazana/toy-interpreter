use super::{Operator, Token, TokenizerResult};
use crate::utils::{split_str_at_start, split_while_alphanumeric, split_while_number};

pub fn tokenize_fun_keyword(input: &str) -> TokenizerResult {
    split_str_at_start("fun", input).map(|(_, rest)| (Token::FunctionKeyword, rest))
}

pub fn tokenize_number(input: &str) -> TokenizerResult {
    split_while_number(input).map(|(num, rest)| (Token::NumberToken(num), rest))
}

pub fn tokenize_left_parens(input: &str) -> TokenizerResult {
    split_str_at_start("(", input).map(|(_, rest)| (Token::LeftParenthesis, rest))
}

pub fn tokenize_right_parens(input: &str) -> TokenizerResult {
    split_str_at_start(")", input).map(|(_, rest)| (Token::RightParenthesis, rest))
}

pub fn tokenize_identifier(input: &str) -> TokenizerResult {
    split_while_alphanumeric(input).map(|(id, rest)| (Token::Identifier(id), rest))
}

pub fn tokenize_assignment(input: &str) -> TokenizerResult {
    split_str_at_start("=", input).map(|(_, rest)| (Token::Assignment, rest))
}

pub fn tokenize_comma(input: &str) -> TokenizerResult {
    split_str_at_start(",", input).map(|(_, rest)| (Token::Comma, rest))
}

pub fn tokenize_addition(input: &str) -> TokenizerResult {
    split_str_at_start("+", input).map(|(_, rest)| (Token::OperatorToken(Operator::Add), rest))
}

pub fn tokenize_subtraction(input: &str) -> TokenizerResult {
    split_str_at_start("-", input).map(|(_, rest)| (Token::OperatorToken(Operator::Subtract), rest))
}

pub fn tokenize_multiplication(input: &str) -> TokenizerResult {
    split_str_at_start("*", input).map(|(_, rest)| (Token::OperatorToken(Operator::Multiply), rest))
}

pub fn tokenize_division(input: &str) -> TokenizerResult {
    split_str_at_start("/", input).map(|(_, rest)| (Token::OperatorToken(Operator::Divide), rest))
}

pub fn tokenize_question_mark(input: &str) -> TokenizerResult {
    split_str_at_start("?", input).map(|(_, rest)| (Token::QuestionMark, rest))
}

pub fn tokenize_colon(input: &str) -> TokenizerResult {
    split_str_at_start(":", input).map(|(_, rest)| (Token::Colon, rest))
}
