use crate::parser::Token;
use crate::utils::partition;

#[derive(Debug)]
pub enum Node {
    Expression(ExpressionNode),
    Statement(StatementNode),
}

#[derive(Debug)]
pub enum ExpressionNode {
    NumericalExpression {
        node_a: Box<ExpressionNode>,
        op: String,
        node_b: Box<ExpressionNode>,
    },
    NumberLiteral {
        value: u32,
    },
    Variable {
        identifier: String,
    },
}

#[derive(Debug)]
pub enum StatementNode {
    AssignmentNode {
        identifier: String,
        expression: ExpressionNode,
    },
}

fn consume_variable_identifier(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
    let (head, rest) = tokens.split_first()?;

    match head {
        Token::Identifier(identifier) => Option::Some((
            ExpressionNode::Variable {
                identifier: identifier.to_string(),
            },
            rest,
        )),
        _ => Option::None,
    }
}

fn consume_number_literal(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
    match tokens.split_first() {
        Option::Some((Token::NumberToken(number_str), rest)) => number_str
            .parse()
            .map(|value| (ExpressionNode::NumberLiteral { value }, rest))
            .ok(),
        _ => Option::None,
    }
}

pub fn consume_math_operation(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
    fn build_math_expression(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
        consume_parenthesis(tokens)
            .or_else(|| consume_number_literal(tokens))
            .or_else(|| consume_variable_identifier(tokens))
    }

    let (node_a, rest_a) = build_math_expression(tokens)?;
    let (maybe_operator, rest_after_op) = rest_a.split_first()?;

    match maybe_operator {
        Token::OperatorToken(op) => {
            let (node_b, rest_b) = build_expression(rest_after_op)?;
            let result_node = ExpressionNode::NumericalExpression {
                node_a: Box::new(node_a),
                node_b: Box::new(node_b),
                op: op.clone(),
            };

            Option::Some((result_node, rest_b))
        }
        _ => Option::None,
    }
}

pub fn consume_parenthesis(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
    let is_right_parenthesis = |token: &Token| match token {
        Token::RightParenthesis => true,
        _ => false,
    };

    match tokens.split_first() {
        Option::Some((Token::LeftParenthesis, rest_tokens)) => {
            let (tokens_in_parens, _, tokens_after_parens) =
                partition(rest_tokens, is_right_parenthesis)?;

            build_expression(tokens_in_parens)
                .filter(|(_, rest)| rest.len() == 0)
                .map(|(node, _)| (node, tokens_after_parens))
        }
        _ => Option::None,
    }
}

pub fn build_expression(tokens: &[Token]) -> Option<(ExpressionNode, &[Token])> {
    consume_math_operation(tokens)
        .or_else(|| consume_parenthesis(tokens))
        .or_else(|| consume_number_literal(tokens))
        .or_else(|| consume_variable_identifier(tokens))
}

pub fn consume_assignemnt(tokens: &[Token]) -> Option<(StatementNode, &[Token])> {
    if tokens.len() < 3 {
        return Option::None;
    }

    match (&tokens[0], &tokens[1], &tokens[2..]) {
        (Token::Identifier(identifier), Token::Assignment, tokens_after_assignment) => {
            let (expression, rest) = build_expression(tokens_after_assignment)?;
            let result_node = StatementNode::AssignmentNode {
                identifier: identifier.clone(),
                expression,
            };

            Option::Some((result_node, rest))
        }
        _ => Option::None,
    }
}

pub fn build_statement(tokens: &[Token]) -> Option<(StatementNode, &[Token])> {
    consume_assignemnt(tokens)
}

pub fn build(tokens: &[Token]) -> Option<(Node, &[Token])> {
    build_statement(tokens)
        .map(|(node, rest)| (Node::Statement(node), rest))
        .or_else(|| build_expression(tokens).map(|(node, rest)| (Node::Expression(node), rest)))
}

pub fn run(tokens: &[Token]) -> Result<Vec<Node>, &str> {
    let mut left_to_parse = tokens;
    let mut nodes: Vec<Node> = Vec::new();

    loop {
        let res = build(left_to_parse);
        match res {
            Option::Some((node, rest)) => {
                left_to_parse = rest;
                nodes.push(node);
            }
            Option::None => {
                break if left_to_parse.len() == 0 {
                    Result::Ok(nodes)
                } else {
                    println!("{:#?}", nodes);
                    println!("{:#?}", left_to_parse);
                    Result::Err("Failed to fully consume the tokens")
                }
            }
        }
    }
}
