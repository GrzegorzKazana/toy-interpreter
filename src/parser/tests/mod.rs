#[allow(unused_imports)]
use super::expressions::ExpressionNode;
#[allow(unused_imports)]
use super::statement::StatementNode;
#[allow(unused_imports)]
use super::{run, Node};
#[allow(unused_imports)]
use crate::tokenizer::Token;

#[test]
fn it_detects_simple_expressions() {
    // "1 + 2 + 3"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::NumericalExpression {
        op: String::from("+"),
        node_a: Box::new(ExpressionNode::NumericalExpression {
            op: String::from("+"),
            node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
            node_b: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
        }),
        node_b: Box::new(ExpressionNode::NumberLiteral { value: 3 }),
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_expressions_with_parenthesis() {
    // "1 + (2 + 3)"
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("3")),
        Token::RightParenthesis,
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::NumericalExpression {
        op: String::from("+"),
        node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
        node_b: Box::new(ExpressionNode::NumericalExpression {
            op: String::from("+"),
            node_a: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
            node_b: Box::new(ExpressionNode::NumberLiteral { value: 3 }),
        }),
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_1() {
    // 1 + 2 * 3
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("*")),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::NumericalExpression {
        op: String::from("+"),
        node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
        node_b: Box::new(ExpressionNode::NumericalExpression {
            op: String::from("*"),
            node_a: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
            node_b: Box::new(ExpressionNode::NumberLiteral { value: 3 }),
        }),
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_2() {
    // 1 * 2 + 3
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("*")),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("3")),
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::NumericalExpression {
        op: String::from("+"),
        node_a: Box::new(ExpressionNode::NumericalExpression {
            op: String::from("*"),
            node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
            node_b: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
        }),
        node_b: Box::new(ExpressionNode::NumberLiteral { value: 3 }),
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_respects_operator_priority_3() {
    // 1 + 2 * 3 * 4
    let input = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("*")),
        Token::NumberToken(String::from("3")),
        Token::OperatorToken(String::from("*")),
        Token::NumberToken(String::from("4")),
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::NumericalExpression {
        op: String::from("+"),
        node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
        node_b: Box::new(ExpressionNode::NumericalExpression {
            op: String::from("*"),
            node_a: Box::new(ExpressionNode::NumericalExpression {
                op: String::from("*"),
                node_a: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
                node_b: Box::new(ExpressionNode::NumberLiteral { value: 3 }),
            }),
            node_b: Box::new(ExpressionNode::NumberLiteral { value: 4 }),
        }),
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_simple_assignment() {
    // "id = 42"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::NumberToken(String::from("42")),
    ];

    let expected_result = vec![Node::Statement(StatementNode::AssignmentNode {
        identifier: String::from("id"),
        expression: ExpressionNode::NumberLiteral { value: 42 },
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_function_call() {
    // "id()"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::FunctionCall {
        identifier: String::from("id"),
        arguments: vec![],
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_function_call_with_arguments() {
    // "id(42, 1 + 2, (a))"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("42")),
        Token::Comma,
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("2")),
        Token::Comma,
        Token::LeftParenthesis,
        Token::Identifier(String::from("a")),
        Token::RightParenthesis,
        Token::RightParenthesis,
    ];

    let expected_result = vec![Node::Expression(ExpressionNode::FunctionCall {
        identifier: String::from("id"),
        arguments: vec![
            ExpressionNode::NumberLiteral { value: 42 },
            ExpressionNode::NumericalExpression {
                node_a: Box::new(ExpressionNode::NumberLiteral { value: 1 }),
                node_b: Box::new(ExpressionNode::NumberLiteral { value: 2 }),
                op: String::from("+"),
            },
            ExpressionNode::Variable {
                identifier: String::from("a"),
            },
        ],
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
