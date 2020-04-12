use super::super::expressions::{build_expression, consume_variable_identifier, ExpressionNode};
use super::{ParsingResult, StatementNode, StatementParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub identifier: String,
    pub arguments: Vec<ExpressionNode>,
    pub expression: ExpressionNode,
}

pub fn consume_function_declaration(tokens: &[Token]) -> StatementParsingResult {
    let maybe_fn_keyword = tokens.get(0)?;
    let maybe_identifier = tokens.get(1)?;
    let rest = &tokens[2..];

    match (maybe_fn_keyword, maybe_identifier) {
        (Token::FunctionKeyword, Token::Identifier(identifier)) => {
            let (arguments, arguments_rest) = consume_arguments(rest)?;
            let (maybe_assignment, assignment_rest) = arguments_rest.split_first()?;

            match maybe_assignment {
                Token::Assignment => {
                    let (expression, expression_rest) = build_expression(assignment_rest)?;
                    let result_node = StatementNode::FunctionDeclaration(FunctionDeclaration {
                        identifier: identifier.clone(),
                        arguments,
                        expression,
                    });

                    Option::Some((result_node, expression_rest))
                }
                _ => Option::None,
            }
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
                let (arg, arg_rest) = consume_variable_identifier(rest_tokens)?;
                args.push(arg);
                rest_tokens = arg_rest;
            }
        };
    }
}
