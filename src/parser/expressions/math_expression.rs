use super::{build_simple_math_expression, ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::Token;

/**
 * Will only consume math operations as long as right operand
 * is an expression with an operand with higher priority.
 * E.g. will consume "1+2*3", but will not "1*2+3".
 */
pub fn consume_math_operation(
    tokens: &[Token],
    maybe_minimal_priority: Option<usize>,
) -> ExpressionParsingResult {
    let minimal_priority = maybe_minimal_priority.unwrap_or(1);
    let (operand, operand_rest) = build_simple_math_expression(tokens)?;

    match operand_rest.split_first() {
        Option::None => Option::Some((operand, operand_rest)),
        Option::Some((Token::OperatorToken(op), operator_rest)) => {
            let priority = operator_to_priority(op);

            if operator_to_priority(op) >= minimal_priority {
                let (operand_b, operand_b_rest) =
                    consume_math_operation(operator_rest, Option::Some(priority + 1))?;
                let result_node = ExpressionNode::NumericalExpression {
                    node_a: Box::new(operand),
                    node_b: Box::new(operand_b),
                    op: op.clone(),
                };

                Option::Some((result_node, operand_b_rest))
            } else {
                Option::Some((operand, operand_rest))
            }
        }
        _ => Option::None,
    }
}

fn operator_to_priority(op: &str) -> usize {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}
