use super::common::{CharTokenizer, RegexTokenizer};
use crate::tokenizer::{Token, Tokenizer, TokenizerResult};

pub struct CombinedTokenizer {
    tokenizers: Vec<Box<dyn Tokenizer<Token>>>,
}

impl CombinedTokenizer {
    pub fn new() -> Self {
        let number_tokenizer = RegexTokenizer::new(r"^(\d+)", Box::new(Token::NumberToken));
        let opeartor_tokenizer = RegexTokenizer::new(r"^(\+|-)", Box::new(Token::OperatorToken));
        let left_paren_tokenizer = CharTokenizer::new('(', Box::new(|_| Token::LeftParenthesis));
        let right_paren_tokenizer = CharTokenizer::new(')', Box::new(|_| Token::RightParenthesis));
        let identifier_tokenizer = RegexTokenizer::new(r"^\w+", Box::new(Token::Identifier));
        let assignment_tokenizer = CharTokenizer::new('=', Box::new(|_| Token::Assignment));

        CombinedTokenizer {
            tokenizers: vec![
                Box::new(number_tokenizer),
                Box::new(opeartor_tokenizer),
                Box::new(left_paren_tokenizer),
                Box::new(right_paren_tokenizer),
                Box::new(identifier_tokenizer),
                Box::new(assignment_tokenizer),
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
