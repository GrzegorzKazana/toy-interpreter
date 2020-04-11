#[allow(unused_imports)]
use super::{run, Token};

#[test]
fn it_tokenizes_numerical_expressions() {
    let input = "1 + (2 + 3)";
    let expected_result = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(String::from("+")),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(String::from("+")),
        Token::NumberToken(String::from("3")),
        Token::RightParenthesis,
    ];

    let result = run(input).unwrap();

    assert_eq!(result, expected_result);
}

#[test]
fn it_tokenizes_assignments() {
    let input = "id = 42";
    let expected_result = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::NumberToken(String::from("42")),
    ];

    let result = run(input).unwrap();

    assert_eq!(result, expected_result);
}

#[test]
fn it_tokenizes_assignment_with_function_call() {
    let input = "id = f(1)";
    let expected_result = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::Identifier(String::from("f")),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("1")),
        Token::RightParenthesis,
    ];

    let result = run(input).unwrap();

    assert_eq!(result, expected_result);
}
