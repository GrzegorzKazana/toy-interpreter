use crate::parser::Token;
use crate::utils::partition;

#[derive(Debug)]
pub enum Node {
    NumericalExpression(NumericalExpressionNode),
    NumberLiteral(NumberLiteralNode),
}

#[derive(Debug)]
pub struct NumericalExpressionNode {
    node_a: Box<Node>,
    op: String,
    node_b: Box<Node>,
}

impl NumericalExpressionNode {
    pub fn new(node_a: Node, node_b: Node, op: String) -> Self {
        NumericalExpressionNode {
            node_a: Box::new(node_a),
            node_b: Box::new(node_b),
            op: op.clone(),
        }
    }
}

#[derive(Debug)]
pub struct NumberLiteralNode {
    value: u32,
}

fn consume_number_literal(tokens: &[Token]) -> Option<(Node, &[Token])> {
    match tokens.split_first() {
        Option::Some((Token::NumberToken(number_str), rest)) => number_str
            .parse()
            .map(|value| (Node::NumberLiteral(NumberLiteralNode { value }), rest))
            .ok(),
        _ => Option::None,
    }
}

pub fn build_expression(tokens: &[Token]) -> Option<(Node, &[Token])> {
    consume_parenthesis(tokens).or_else(|| consume_number_literal(tokens))
}

pub fn consume_math_operation(tokens: &[Token]) -> Option<(Node, &[Token])> {
    let (node_a, rest_a) = build_expression(tokens)?;
    let (maybe_operator, rest_after_op) = rest_a.split_first()?;

    match maybe_operator {
        Token::OperatorToken(op) => {
            let (node_b, rest_b) = build(rest_after_op)?;
            let result_node = NumericalExpressionNode::new(node_a, node_b, op.to_string());

            Option::Some((Node::NumericalExpression(result_node), rest_b))
        }
        _ => Option::None,
    }
}

pub fn consume_parenthesis(tokens: &[Token]) -> Option<(Node, &[Token])> {
    let is_right_parenthesis = |token: &Token| match token {
        Token::RightParenthesis => true,
        _ => false,
    };

    match tokens.split_first() {
        Option::Some((Token::LeftParenthesis, rest_tokens)) => {
            let (tokens_in_parens, _, tokens_after_parens) =
                partition(rest_tokens, is_right_parenthesis)?;

            build(tokens_in_parens)
                .filter(|(_, rest)| rest.len() == 0)
                .map(|(node, _)| (node, tokens_after_parens))
        }
        _ => Option::None,
    }
}

pub fn build(tokens: &[Token]) -> Option<(Node, &[Token])> {
    consume_math_operation(tokens)
        .or_else(|| consume_parenthesis(tokens))
        .or_else(|| consume_number_literal(tokens))
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
                    Result::Err("Failed to fully consume the tokens")
                }
            }
        }
    }
}
