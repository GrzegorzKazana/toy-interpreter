use regex::Regex;

fn main() {
    let input = "2 + 2";
    let combined_parser = build_parser();

    let mut parsing_input = String::from(input);
    let mut v: Vec<Token> = Vec::new();

    loop {
        let res = combined_parser.parse(&parsing_input);
        match res {
            Option::Some((matched, rest)) => {
                v.push(matched);
                parsing_input = rest;
            }
            Option::None => {
                println!("{:#?}", parsing_input);
                println!("{:#?}", v);
                break;
            }
        };
    }
}

#[derive(Debug)]
enum Token {
    NumberToken(String),
    OperatorToken(String),
}

type SourceRest = String;
type ParsingResult<T> = Option<(T, SourceRest)>;

pub trait Parser<T> {
    fn parse(&self, input: &String) -> ParsingResult<T>;
}

struct RegexParser {
    re: Regex,
}
impl Parser<String> for RegexParser {
    fn parse(&self, input: &String) -> ParsingResult<String> {
        self.re
            .find(input)
            .map(|capture| split_str_trim(input, capture.end()))
    }
}

struct NumberParser {
    _internal_parser: RegexParser,
}
impl NumberParser {
    fn new() -> NumberParser {
        NumberParser {
            _internal_parser: RegexParser {
                re: Regex::new(r"^(\d+)").unwrap(),
            },
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

struct OperatorParser {
    _internal_parser: RegexParser,
}
impl OperatorParser {
    fn new() -> OperatorParser {
        OperatorParser {
            _internal_parser: RegexParser {
                re: Regex::new(r"^(\+|-)").unwrap(),
            },
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

struct CombinedParser {
    parsers: Vec<Box<dyn Parser<Token>>>,
}
impl Parser<Token> for CombinedParser {
    fn parse(&self, input: &String) -> ParsingResult<Token> {
        self.parsers.iter().fold(Option::None, |acc, parser| {
            acc.or_else(|| parser.parse(input))
        })
    }
}

struct CharParser {
    character: char,
}
impl Parser<String> for CharParser {
    fn parse(&self, input: &String) -> ParsingResult<String> {
        if input.starts_with(self.character) {
            Option::Some(split_str_trim(input, 1))
        } else {
            Option::None
        }
    }
}

fn build_parser() -> impl Parser<Token> {
    CombinedParser {
        parsers: vec![
            Box::new(NumberParser::new()),
            Box::new(OperatorParser::new()),
        ],
    }
}

fn split_str_at(input: &String, idx: usize) -> (String, String) {
    let head = input.chars().take(idx).collect();
    let tail = input.chars().skip(idx).collect();

    (head, tail)
}

fn split_str_trim(input: &String, idx: usize) -> (String, String) {
    let (head, tail) = split_str_at(input, idx);

    (head, tail.trim().to_string())
}
