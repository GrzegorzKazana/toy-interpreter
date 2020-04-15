#[cfg(test)]
mod math_expression_tests {
    use crate::parser::expressions::*;
    use crate::parser::{Node, Parser, Program};
    use crate::tokenizer::{Operator, Token};

    #[test]
    fn it_detects_simple_expressions() {
        let mock_parser = Parser {};
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

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_detects_expressions_with_parenthesis() {
        let mock_parser = Parser {};
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

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_respects_operator_priority_1() {
        let mock_parser = Parser {};
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

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_respects_operator_priority_2() {
        let mock_parser = Parser {};
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

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_respects_operator_priority_3() {
        let mock_parser = Parser {};
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
                        node_a: Box::new(ExpressionNode::NumericalExpression(
                            NumericalExpression {
                                op: Operator::Multiply,
                                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral {
                                    value: 2,
                                })),
                                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral {
                                    value: 3,
                                })),
                            },
                        )),
                        node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 4 })),
                    })),
                },
            ))],
        };

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_detects_expression_with_subtraction() {
        let mock_parser = Parser {};
        // "1 - 2"
        let input = vec![
            Token::NumberToken(String::from("1")),
            Token::OperatorToken(Operator::Subtract),
            Token::NumberToken(String::from("2")),
        ];

        let expected_result = Program {
            body: vec![Node::Expression(ExpressionNode::NumericalExpression(
                NumericalExpression {
                    op: Operator::Subtract,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                },
            ))],
        };

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_detects_signed_number_literal() {
        let mock_parser = Parser {};
        // -1
        let input = vec![
            Token::OperatorToken(Operator::Subtract),
            Token::NumberToken(String::from("1")),
        ];
        let expected_result = Program {
            body: vec![Node::Expression(ExpressionNode::Negation(Negation {
                expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
            }))],
        };

        let result = mock_parser.parse(&input).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_detects_expression_with_negative_numer() {
        let mock_parser = Parser {};
        // "- 1 - 2"
        let input = vec![
            Token::OperatorToken(Operator::Subtract),
            Token::NumberToken(String::from("1")),
            Token::OperatorToken(Operator::Subtract),
            Token::NumberToken(String::from("2")),
        ];

        let expected_result = Program {
            body: vec![Node::Expression(ExpressionNode::NumericalExpression(
                NumericalExpression {
                    op: Operator::Subtract,
                    node_a: Box::new(ExpressionNode::Negation(Negation {
                        expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral {
                            value: 1,
                        })),
                    })),
                    node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
                },
            ))],
        };

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn it_detects_expression_with_parenthesised_negative_numer() {
        let mock_parser = Parser {};
        // "1 + (-2)"
        let input = vec![
            Token::NumberToken(String::from("1")),
            Token::OperatorToken(Operator::Add),
            Token::LeftParenthesis,
            Token::OperatorToken(Operator::Subtract),
            Token::NumberToken(String::from("2")),
            Token::RightParenthesis,
        ];

        let expected_result = Program {
            body: vec![Node::Expression(ExpressionNode::NumericalExpression(
                NumericalExpression {
                    op: Operator::Add,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                    node_b: Box::new(ExpressionNode::Negation(Negation {
                        expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral {
                            value: 2,
                        })),
                    })),
                },
            ))],
        };

        let parsing_result = mock_parser.parse(&input).unwrap();

        assert_eq!(parsing_result, expected_result);
    }
}
