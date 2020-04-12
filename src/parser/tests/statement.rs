#[allow(unused_imports)]
use crate::parser::expressions::*;
#[allow(unused_imports)]
use crate::parser::statement::StatementNode;
#[allow(unused_imports)]
use crate::parser::{run, Node};
#[allow(unused_imports)]
use crate::tokenizer::Token;

#[test]
fn it_detects_simple_assignment() {
    // "id = 42"
    let input = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::NumberToken(String::from("42")),
    ];

    let expected_result = Node::Program {
        body: vec![Node::Statement(StatementNode::AssignmentNode {
            identifier: String::from("id"),
            expression: ExpressionNode::NumberLiteral(NumberLiteral { value: 42 }),
        })],
    };

    let parsing_result = run(&input).unwrap();

    assert_eq!(parsing_result, expected_result);
}
