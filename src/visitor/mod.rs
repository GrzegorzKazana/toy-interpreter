use crate::tokenizer::{Token, Operator};
use crate::parser::{Node, ExpressionNode, StatementNode};


pub fn visit(visitor: Fn(&Node) -> ()) {

}

fn visitor(node: Node) {
    match node {
        Node::Program => 
    }
}