use super::super::ParsingResult;
use crate::tokenizer::Token;

type ArgumentConsumer<T> = Box<dyn Fn(&[Token]) -> Option<(T, &[Token])>>;

pub fn consume_arguments<T>(
    tokens: &[Token],
    argument_consumer: ArgumentConsumer<T>,
) -> ParsingResult<Vec<T>> {
    let (head, rest) = tokens.split_first()?;
    if !matches!(head, Token::LeftParenthesis) {
        return Option::None;
    }
    let mut args: Vec<T> = Vec::new();
    let mut rest_tokens = rest;

    loop {
        let (rest_head, rest_tail) = rest_tokens.split_first()?;

        match rest_head {
            Token::RightParenthesis => return Option::Some((args, rest_tail)),
            Token::Comma => {
                rest_tokens = rest_tail;
                continue;
            }
            _ => {
                let (arg, arg_rest) = argument_consumer(rest_tokens)?;
                args.push(arg);
                rest_tokens = arg_rest;
            }
        };
    }
}
