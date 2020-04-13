#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::{Node, Parser, Program};
#[allow(unused_imports)]
use crate::tokenizer::{Operator, Token};

#[test]
fn it_detects_simple_terenary() {
    let mock_parser = Parser {};
    // "1 ? 2 : 3"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::QuestionMark,
        Token::NumberToken(String::from("2")),
        Token::Colon,
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::Terenary(Terenary {
            condition: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
            value: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
            alternative: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
        }))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_terenary_with_expression() {
    let mock_parser = Parser {};
    // "1 + 2 ? -3 : 3"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("2")),
        Token::QuestionMark,
        Token::OperatorToken(Operator::Subtract),
        Token::NumberToken(String::from("3")),
        Token::Colon,
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::Terenary(Terenary {
            condition: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
            })),
            value: Box::new(ExpressionNode::Negation(Negation {
                expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            })),
            alternative: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
        }))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
