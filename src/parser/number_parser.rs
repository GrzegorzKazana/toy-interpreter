use super::common::RegexParser;
use crate::parser::{Parser, ParsingResult, Token};

pub struct NumberParser {
    _internal_parser: RegexParser,
}

impl NumberParser {
    pub fn new() -> NumberParser {
        NumberParser {
            _internal_parser: RegexParser::new(r"^(\d+)"),
        }
    }
}

impl Parser<Token> for NumberParser {
    fn parse(&self, input: &String) -> ParsingResult<Token> {
        self._internal_parser
            .parse(input)
            .map(|(content, rest)| (Token::NumberToken(content), rest))
    }
}
