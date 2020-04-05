use regex::Regex;

use super::{Parser, ParsingResult};
use crate::utils;

pub struct RegexParser<T> {
    re: Regex,
    result_formatter: Box<dyn Fn(String) -> T>,
}

impl<T> RegexParser<T> {
    pub fn new(re_str: &str, result_formatter: Box<dyn Fn(String) -> T>) -> Self {
        RegexParser {
            re: Regex::new(re_str).unwrap(),
            result_formatter,
        }
    }
}

impl<T> Parser<T> for RegexParser<T> {
    fn parse(&self, input: &String) -> ParsingResult<T> {
        self.re
            .find(input)
            .map(|capture| utils::split_str_trim(input, capture.end()))
            .map(|(content, rest)| ((self.result_formatter)(content), rest))
    }
}

pub struct CharParser<T> {
    pub character: char,
    result_formatter: Box<dyn Fn(String) -> T>,
}

impl<T> CharParser<T> {
    pub fn new(character: char, result_formatter: Box<dyn Fn(String) -> T>) -> Self {
        CharParser {
            character,
            result_formatter,
        }
    }
}

impl<T> Parser<T> for CharParser<T> {
    fn parse(&self, input: &String) -> ParsingResult<T> {
        if input.starts_with(self.character) {
            Option::Some(utils::split_str_trim(input, 1))
                .map(|(content, rest)| ((self.result_formatter)(content), rest))
        } else {
            Option::None
        }
    }
}
