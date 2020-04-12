#[allow(unused_imports)]
use crate::parser::expressions::ExpressionNode;
#[allow(unused_imports)]
use crate::parser::{run, Node};
#[allow(unused_imports)]
use crate::tokenizer::{Operator, Token};

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
        Token::OperatorToken(Operator::Add),
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
                op: Operator::Add,
            },
            ExpressionNode::Variable {
                identifier: String::from("a"),
            },
        ],
    })];

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
