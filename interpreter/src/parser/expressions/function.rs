use super::super::common::consume_arguments;
use super::{build_expression, ExpressionNode, ExpressionParsingResult, ParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub identifier: String,
    pub arguments: Vec<ExpressionNode>,
}

pub fn consume_function_call(tokens: &[Token]) -> ExpressionParsingResult {
    let (maybe_identifier, rest) = tokens.split_first()?;
    match maybe_identifier {
        Token::Identifier(identifier) => {
            let (arguments, rest_after_args) = consume_call_arguments(rest)?;
            let result_node = ExpressionNode::FunctionCall(FunctionCall {
                identifier: identifier.clone(),
                arguments,
            });

            Option::Some((result_node, rest_after_args))
        }
        _ => Option::None,
    }
}

fn consume_call_arguments(tokens: &[Token]) -> ParsingResult<Vec<ExpressionNode>> {
    consume_arguments(tokens, Box::new(build_expression))
}
