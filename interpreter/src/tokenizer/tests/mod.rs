#[cfg(test)]
mod tokenizer_tests {
    use crate::tokenizer::{InputTokenizer, Operator, Token};

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

    #[test]
    fn it_tokenizes_function_definition() {
        let mock_tokenizer = InputTokenizer {};
        let input = "fun sum(a, b) = a + b";
        let expected_result = vec![
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

        let result = mock_tokenizer.tokenize(input).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_tokenizes_terenary() {
        let mock_tokenizer = InputTokenizer {};
        let input = "a ? 2 : 1";
        let expected_result = vec![
            Token::Identifier(String::from("a")),
            Token::QuestionMark,
            Token::NumberToken(String::from("2")),
            Token::Colon,
            Token::NumberToken(String::from("1")),
        ];

        let result = mock_tokenizer.tokenize(input).unwrap();

        assert_eq!(result, expected_result);
    }
}
