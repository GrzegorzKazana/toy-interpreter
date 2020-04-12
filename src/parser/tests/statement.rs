#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::statements::*;
#[allow(unused_imports)]
use crate::parser::{Node, Parser, Program};
#[allow(unused_imports)]
use crate::tokenizer::{Operator, Token};

#[test]
fn it_detects_simple_assignment() {
    let mock_parser = Parser {};
    // "id = 42"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::NumberToken(String::from("42")),
    ];

    let expected_result = Program {
        body: vec![Node::Statement(StatementNode::Assignment(Assignment {
            identifier: String::from("id"),
            expression: ExpressionNode::NumberLiteral(NumberLiteral { value: 42 }),
        }))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}

#[test]
fn it_detects_function_declaration() {
    let mock_parser = Parser {};
    // "fun sum(a, b) = a + b";
    let input = vec![
        Token::FunctionKeyword,
        Token::Identifier(String::from("sum")),
        Token::LeftParenthesis,
        Token::Identifier(String::from("a")),
        Token::Comma,
        Token::Identifier(String::from("b")),
        Token::RightParenthesis,
        Token::Assignment,
        Token::Identifier(String::from("a")),
        Token::OperatorToken(Operator::Add),
        Token::Identifier(String::from("b")),
    ];

    let expected_result = Program {
        body: vec![Node::Statement(StatementNode::FunctionDeclaration(
            FunctionDeclaration {
                identifier: String::from("sum"),
                expression: ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Add,
                    node_a: Box::new(ExpressionNode::Variable(Variable {
                        identifier: String::from("a"),
                    })),
                    node_b: Box::new(ExpressionNode::Variable(Variable {
                        identifier: String::from("b"),
                    })),
                }),
                arguments: vec![
                    ExpressionNode::Variable(Variable {
                        identifier: String::from("a"),
                    }),
                    ExpressionNode::Variable(Variable {
                        identifier: String::from("b"),
                    }),
                ],
            },
        ))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
