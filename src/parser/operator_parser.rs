use super::common::RegexParser;
use crate::parser::{Parser, ParsingResult, Token};

pub struct OperatorParser {
    _internal_parser: RegexParser,
}

impl OperatorParser {
    pub fn new() -> OperatorParser {
        OperatorParser {
            _internal_parser: RegexParser::new(r"^(\+|-)"),
        }
    }
}

impl Parser<Token> for OperatorParser {
    fn parse(&self, input: &String) -> ParsingResult<Token> {
        self._internal_parser
            .parse(input)
            .map(|(content, rest)| (Token::OperatorToken(content), rest))
    }
}
