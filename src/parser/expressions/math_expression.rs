use super::{
    build_expression, consume_function_call, consume_number_literal, consume_parenthesis,
    consume_variable_identifier, ExpressionNode, ExpressionParsingResult,
};
use crate::tokenizer::Token;

pub fn consume_math_operation(tokens: &[Token]) -> ExpressionParsingResult {
    fn build_math_expression(tokens: &[Token]) -> ExpressionParsingResult {
        consume_function_call(tokens)
            .or_else(|| consume_parenthesis(tokens))
            .or_else(|| consume_number_literal(tokens))
            .or_else(|| consume_variable_identifier(tokens))
    }

    let (node_a, rest_a) = build_math_expression(tokens)?;
    let (maybe_operator, rest_after_op) = rest_a.split_first()?;

    match maybe_operator {
        Token::OperatorToken(op) => {
            let (node_b, rest_b) = build_expression(rest_after_op)?;
            let result_node = ExpressionNode::NumericalExpression {
                node_a: Box::new(node_a),
                node_b: Box::new(node_b),
                op: op.clone(),
            };

            Option::Some((result_node, rest_b))
        }
        _ => Option::None,
    }
}
