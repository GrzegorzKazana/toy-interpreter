#[allow(unused_imports)]
use super::{InputTokenizer, Operator, Token};

#[test]
fn it_tokenizes_numerical_expressions() {
    let mock_tokenizer = InputTokenizer {};
    let input = "1 + (2 + 3)";
    let expected_result = vec![
        Token::NumberToken(String::from("1")),
        Token::OperatorToken(Operator::Add),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("2")),
        Token::OperatorToken(Operator::Add),
        Token::NumberToken(String::from("3")),
        Token::RightParenthesis,
    ];

    let result = mock_tokenizer.tokenize(input).unwrap();

    assert_eq!(result, expected_result);
}

#[test]
fn it_tokenizes_assignments() {
    let mock_tokenizer = InputTokenizer {};
    let input = "id = 42";
    let expected_result = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::NumberToken(String::from("42")),
    ];

    let result = mock_tokenizer.tokenize(input).unwrap();

    assert_eq!(result, expected_result);
}

#[test]
fn it_tokenizes_assignment_with_function_call() {
    let mock_tokenizer = InputTokenizer {};
    let input = "id = f(1)";
    let expected_result = vec![
        Token::Identifier(String::from("id")),
        Token::Assignment,
        Token::Identifier(String::from("f")),
        Token::LeftParenthesis,
        Token::NumberToken(String::from("1")),
        Token::RightParenthesis,
    ];

    let result = mock_tokenizer.tokenize(input).unwrap();

    assert_eq!(result, expected_result);
}
