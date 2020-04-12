#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::{Node, Parser, Program};
#[allow(unused_imports)]
use crate::tokenizer::{Operator, Token};

#[test]
fn it_detects_function_call() {
    let mock_parser = Parser {};
    // "id()"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
    ];

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::FunctionCall(
            FunctionCall {
                identifier: String::from("id"),
                arguments: vec![],
            },
        ))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_function_call_with_arguments() {
    let mock_parser = Parser {};
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

    let expected_result = Program {
        body: vec![Node::Expression(ExpressionNode::FunctionCall(
            FunctionCall {
                identifier: String::from("id"),
                arguments: vec![
                    ExpressionNode::NumberLiteral(NumberLiteral { value: 42 }),
                    ExpressionNode::NumericalExpression(NumericalExpression {
                        node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                        node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                        op: Operator::Add,
                    }),
                    ExpressionNode::Variable(Variable {
                        identifier: String::from("a"),
                    }),
                ],
            },
        ))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
