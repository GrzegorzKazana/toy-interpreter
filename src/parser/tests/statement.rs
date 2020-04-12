#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::statements::*;
#[allow(unused_imports)]
use crate::parser::{Parser, Node, Program};
#[allow(unused_imports)]
use crate::tokenizer::Token;

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
        body: vec![Node::Statement(StatementNode::AssignmentNode(
            AssignmentNode {
                identifier: String::from("id"),
                expression: ExpressionNode::NumberLiteral(NumberLiteral { value: 42 }),
            },
        ))],
    };

    let parsing_result = mock_parser.parse(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}