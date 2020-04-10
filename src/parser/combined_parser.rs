use super::common::{CharParser, RegexParser};
use crate::parser::{Parser, ParsingResult, Token};

pub struct CombinedParser {
    parsers: Vec<Box<dyn Parser<Token>>>,
}

impl CombinedParser {
    pub fn new() -> Self {
        let number_parser = RegexParser::new(r"^(\d+)", Box::new(Token::NumberToken));
        let opeartor_parser = RegexParser::new(r"^(\+|-)", Box::new(Token::OperatorToken));
        let left_paren_parser = CharParser::new('(', Box::new(|_| Token::LeftParenthesis));
        let right_paren_parser = CharParser::new(')', Box::new(|_| Token::RightParenthesis));
        let identifier_parser = RegexParser::new(r"^\w+", Box::new(Token::Identifier));
        let assignment_parser = CharParser::new('=', Box::new(|_| Token::Assignment));

        CombinedParser {
            parsers: vec![
                Box::new(number_parser),
                Box::new(opeartor_parser),
                Box::new(left_paren_parser),
                Box::new(right_paren_parser),
                Box::new(identifier_parser),
                Box::new(assignment_parser),
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
