use super::{consume_math_expression, ExpressionNode, ExpressionParsingResult};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Terenary {
    pub condition: Box<ExpressionNode>,
    pub value: Box<ExpressionNode>,
    pub alternative: Box<ExpressionNode>,
}

pub fn consume_terenary(tokens: &[Token]) -> ExpressionParsingResult {
    let (condition, rest) = consume_math_expression(tokens)?;
    let (maybe_question, question_rest) = rest.split_first()?;

    if let Token::QuestionMark = maybe_question {
        let (value_expr, expr_rest) = consume_math_expression(question_rest)?;
        let (maybe_colon, colon_rest) = expr_rest.split_first()?;

        if let Token::Colon = maybe_colon {
            let (alternative, alt_rest) = consume_math_expression(colon_rest)?;
            let result_node = ExpressionNode::Terenary(Terenary {
                condition: Box::new(condition),
                value: Box::new(value_expr),
                alternative: Box::new(alternative),
            });

            return Option::Some((result_node, alt_rest));
        }
    }

    Option::None
}
