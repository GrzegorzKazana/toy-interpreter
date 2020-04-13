use super::common::{CharTokenizer, RegexTokenizer};
use crate::tokenizer::{Operator, Token, Tokenizer, TokenizerResult};

pub struct CombinedTokenizer {
    tokenizers: Vec<Box<dyn Tokenizer<Token>>>,
}

impl CombinedTokenizer {
    pub fn new() -> Self {
        let fun_keyword_tokenizer =
            RegexTokenizer::new(r"^fun", Box::new(|_| Token::FunctionKeyword));
        let number_tokenizer = RegexTokenizer::new(r"^(\d+)", Box::new(Token::NumberToken));
        let left_paren_tokenizer = CharTokenizer::new('(', Box::new(|_| Token::LeftParenthesis));
        let right_paren_tokenizer = CharTokenizer::new(')', Box::new(|_| Token::RightParenthesis));
        let identifier_tokenizer = RegexTokenizer::new(r"^\w+", Box::new(Token::Identifier));
        let assignment_tokenizer = CharTokenizer::new('=', Box::new(|_| Token::Assignment));
        let comma_tokenizer = CharTokenizer::new(',', Box::new(|_| Token::Comma));
        let add_op_tokenizer =
            CharTokenizer::new('+', Box::new(|_| (Token::OperatorToken(Operator::Add))));
        let question_tokenizer = CharTokenizer::new('?', Box::new(|_| Token::QuestionMark));
        let colon_tokenizer = CharTokenizer::new(':', Box::new(|_| Token::Colon));
        let add_sub_tokenizer = CharTokenizer::new(
            '-',
            Box::new(|_| (Token::OperatorToken(Operator::Subtract))),
        );
        let add_mult_tokenizer = CharTokenizer::new(
            '*',
            Box::new(|_| (Token::OperatorToken(Operator::Multiply))),
        );
        let add_div_tokenizer =
            CharTokenizer::new('/', Box::new(|_| (Token::OperatorToken(Operator::Divide))));

        CombinedTokenizer {
            tokenizers: vec![
                Box::new(fun_keyword_tokenizer),
                Box::new(number_tokenizer),
                Box::new(add_op_tokenizer),
                Box::new(add_sub_tokenizer),
                Box::new(add_mult_tokenizer),
                Box::new(add_div_tokenizer),
                Box::new(left_paren_tokenizer),
                Box::new(right_paren_tokenizer),
                Box::new(identifier_tokenizer),
                Box::new(assignment_tokenizer),
                Box::new(comma_tokenizer),
                Box::new(question_tokenizer),
                Box::new(colon_tokenizer),
            ],
        }
    }
}

impl Tokenizer<Token> for CombinedTokenizer {
    fn tokenize(&self, input: &String) -> TokenizerResult<Token> {
        self.tokenizers.iter().fold(Option::None, |acc, tokenizer| {
            acc.or_else(|| tokenizer.tokenize(input))
        })
    }
}
