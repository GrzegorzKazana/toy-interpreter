use regex::Regex;

use super::{Tokenizer, TokenizerResult};
use crate::utils;

pub struct RegexTokenizer<T> {
    re: Regex,
    result_formatter: Box<dyn Fn(String) -> T>,
}

impl<T> RegexTokenizer<T> {
    pub fn new(re_str: &str, result_formatter: Box<dyn Fn(String) -> T>) -> Self {
        RegexTokenizer {
            re: Regex::new(re_str).unwrap(),
            result_formatter,
        }
    }
}

impl<T> Tokenizer<T> for RegexTokenizer<T> {
    fn tokenize(&self, input: &String) -> TokenizerResult<T> {
        self.re
            .find(input)
            .map(|capture| utils::split_str_trim(input, capture.end()))
            .map(|(content, rest)| ((self.result_formatter)(content), rest))
    }
}

pub struct CharTokenizer<T> {
    pub character: char,
    result_formatter: Box<dyn Fn(String) -> T>,
}

impl<T> CharTokenizer<T> {
    pub fn new(character: char, result_formatter: Box<dyn Fn(String) -> T>) -> Self {
        CharTokenizer {
            character,
            result_formatter,
        }
    }
}

impl<T> Tokenizer<T> for CharTokenizer<T> {
    fn tokenize(&self, input: &String) -> TokenizerResult<T> {
        if input.starts_with(self.character) {
            Option::Some(utils::split_str_trim(input, 1))
                .map(|(content, rest)| ((self.result_formatter)(content), rest))
        } else {
            Option::None
        }
    }
}
