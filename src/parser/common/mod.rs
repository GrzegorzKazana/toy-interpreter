use regex::Regex;

use super::{Parser, ParsingResult};
use crate::utils;

pub struct RegexParser {
    re: Regex,
}
impl RegexParser {
    pub fn new(re_str: &str) -> Self {
        RegexParser {
            re: Regex::new(re_str).unwrap(),
        }
    }
}
impl Parser<String> for RegexParser {
    fn parse(&self, input: &String) -> ParsingResult<String> {
        self.re
            .find(input)
            .map(|capture| utils::split_str_trim(input, capture.end()))
    }
}

pub struct CharParser {
    character: char,
}
impl Parser<String> for CharParser {
    fn parse(&self, input: &String) -> ParsingResult<String> {
        if input.starts_with(self.character) {
            Option::Some(utils::split_str_trim(input, 1))
        } else {
            Option::None
        }
    }
}
