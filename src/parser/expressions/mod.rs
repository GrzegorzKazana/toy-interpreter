mod function;
mod identifier;
mod literal;
mod math_expression;
mod parenthesis;

use super::ParsingResult;
use crate::tokenizer::Token;

use function::consume_function_call;
use identifier::consume_variable_identifier;
use literal::consume_number_literal;
use math_expression::consume_math_operation;
use parenthesis::consume_parenthesis;

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    NumericalExpression {
        node_a: Box<ExpressionNode>,
        op: String,
        node_b: Box<ExpressionNode>,
    },
    NumberLiteral {
        value: u32,
    },
    Variable {
        identifier: String,
    },
    FunctionCall {
        identifier: String,
        arguments: Vec<ExpressionNode>,
    },
}

type ExpressionParsingResult<'a> = ParsingResult<'a, ExpressionNode>;

pub fn build_expression(tokens: &[Token]) -> ExpressionParsingResult {
    consume_math_operation(tokens)
        .or_else(|| consume_function_call(tokens))
        .or_else(|| consume_parenthesis(tokens))
        .or_else(|| consume_number_literal(tokens))
        .or_else(|| consume_variable_identifier(tokens))
}
