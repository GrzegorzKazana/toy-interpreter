mod function;
mod identifier;
mod literal;
mod math_expression;
mod parenthesis;

use super::ParsingResult;
use crate::tokenizer::Token;

use function::consume_function_call;
pub use function::FunctionCall;
use identifier::consume_variable_identifier;
pub use identifier::consume_variable_identifier_helper;
pub use identifier::Variable;
use literal::consume_number_literal;
pub use literal::NumberLiteral;
use math_expression::consume_math_expression;
pub use math_expression::NumericalExpression;
use parenthesis::consume_parenthesis;

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    NumericalExpression(NumericalExpression),
    NumberLiteral(NumberLiteral),
    Variable(Variable),
    FunctionCall(FunctionCall),
}

type ExpressionParsingResult<'a> = ParsingResult<'a, ExpressionNode>;

fn build_simple_math_expression(tokens: &[Token]) -> ExpressionParsingResult {
    consume_function_call(tokens)
        .or_else(|| consume_parenthesis(tokens))
        .or_else(|| consume_number_literal(tokens))
        .or_else(|| consume_variable_identifier(tokens))
}

pub fn build_expression(tokens: &[Token]) -> ExpressionParsingResult {
    consume_math_expression(tokens, 1)
}
