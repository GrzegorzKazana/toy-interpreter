#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::{run, Node, Program};
#[allow(unused_imports)]
use crate::tokenizer::{Operator, Token};

#[test]
fn it_detects_simple_expressions() {
    // "1 + 2 + 3"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::NumericalExpression(
            NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Add,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                })),
                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            },
        ))],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_expressions_with_parenthesis() {
    // "1 + (2 + 3)"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("3")),
        Token::RightParenthesis,
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::NumericalExpression(
            NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Add,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
                })),
            },
        ))],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_1() {
    // 1 + 2 * 3
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Multiply),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::NumericalExpression(
            NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Multiply,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
                })),
            },
        ))],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_2() {
    // 1 * 2 + 3
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Multiply),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::NumericalExpression(
            NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Multiply,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                })),
                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            },
        ))],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_3() {
    // 1 + 2 * 3 * 4
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Multiply),
        Token::NumberToken(String::from("3")),
        Token::OperatorToken(Operator::Multiply),
        Token::NumberToken(String::from("4")),
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::NumericalExpression(
            NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Multiply,
                    node_a: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                        op: Operator::Multiply,
                        node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                        node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
                    })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 4 })),
                })),
            },
        ))],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
