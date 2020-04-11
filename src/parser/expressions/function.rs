use super::{build_expression, ExpressionNode, ExpressionParsingResult, ParsingResult};
use crate::tokenizer::Token;

pub fn consume_function_call(tokens: &[Token]) -> ExpressionParsingResult {
    let (maybe_identifier, rest) = tokens.split_first()?;
    match maybe_identifier {
        Token::Identifier(identifier) => {
            let (arguments, rest_after_args) = consume_arguments(rest)?;
            let result_node = ExpressionNode::FunctionCall {
                identifier: identifier.clone(),
                arguments,
            };

            Option::Some((result_node, rest_after_args))
        }
        _ => Option::None,
    }
}

fn consume_arguments(tokens: &[Token]) -> ParsingResult<Vec<ExpressionNode>> {
    let (head, rest) = tokens.split_first()?;
    if !matches!(head, Token::LeftParenthesis) {
        return Option::None;
    }
    let mut args: Vec<ExpressionNode> = Vec::new();
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
                let (arg, arg_rest) = build_expression(rest_tokens)?;
                args.push(arg);
                rest_tokens = arg_rest;
            }
        };
    }
}
