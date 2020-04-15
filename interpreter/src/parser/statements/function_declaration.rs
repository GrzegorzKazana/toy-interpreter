use super::super::common::consume_arguments;
use super::super::expressions::{
    build_expression, consume_variable_identifier_helper, ExpressionNode, Variable,
};
use super::{ParsingResult, StatementNode, StatementParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub identifier: String,
    pub arguments: Vec<Variable>,
    pub expression: ExpressionNode,
}

pub fn consume_function_declaration(tokens: &[Token]) -> StatementParsingResult {
    let maybe_fn_keyword = tokens.get(0)?;
    let maybe_identifier = tokens.get(1)?;
    let rest = &tokens[2..];

    match (maybe_fn_keyword, maybe_identifier) {
        (Token::FunctionKeyword, Token::Identifier(identifier)) => {
            let (arguments, arguments_rest) = consume_declaration_arguments(rest)?;
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

fn consume_declaration_arguments(tokens: &[Token]) -> ParsingResult<Vec<Variable>> {
    consume_arguments(tokens, Box::new(consume_variable_identifier_helper))
}
