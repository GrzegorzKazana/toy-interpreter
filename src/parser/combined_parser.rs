use super::number_parser::NumberParser;
use super::operator_parser::OperatorParser;
use crate::parser::{Parser, ParsingResult, Token};

pub struct CombinedParser {
    parsers: Vec<Box<dyn Parser<Token>>>,
}

impl CombinedParser {
    pub fn new() -> Self {
        CombinedParser {
            parsers: vec![
                Box::new(NumberParser::new()),
                Box::new(OperatorParser::new()),
            ],
        }
    }
}

impl Parser<Token> for CombinedParser {
    fn parse(&self, input: &String) -> ParsingResult<Token> {
        self.parsers.iter().fold(Option::None, |acc, parser| {
            acc.or_else(|| parser.parse(input))
        })
    }
}
