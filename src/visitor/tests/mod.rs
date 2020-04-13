use super::{Context, VisitExpressionResult, VisitStatementResult, Visitor};
#[allow(unused_imports)]
use crate::parser::expressions::{FunctionCall, NumberLiteral, NumericalExpression, Variable};
use crate::parser::statements::{Assignment, FunctionDeclaration};
#[allow(unused_imports)]
use crate::parser::ExpressionNode;
#[allow(unused_imports)]
use crate::tokenizer::Operator;

struct MockVisitor {}
impl Visitor for MockVisitor {
    fn visit_var(&self, _: &Variable, _: Context) -> VisitExpressionResult {
        Result::Err(String::from("Not implemented"))
    }
    fn visit_fn_call(&self, _: &FunctionCall, _: Context) -> VisitExpressionResult {
        Result::Err(String::from("Not implemented"))
    }
    fn visit_assignment(&mut self, _: &Assignment) -> VisitStatementResult {
        Result::Err(String::from("Not implemented"))
    }
    fn visit_fn_declaration(&mut self, _: &FunctionDeclaration) -> VisitStatementResult {
        Result::Err(String::from("Not implemented"))
    }
}

#[test]
fn it_visits_number_literals() {
    let visitor = MockVisitor {};
    let number_literal = NumberLiteral { value: 42 };

    let exprected_result = Result::Ok(42);

    let result = visitor.visit_num(&number_literal);

    assert_eq!(result, exprected_result);
}

#[test]
fn it_visits_simple_math_expression() {
    let visitor = MockVisitor {};
    let math_expr = NumericalExpression {
        op: Operator::Add,
        node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
        node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
    };

    let exprected_result = Result::Ok(5);

    let result = visitor.visit_math_expr(&math_expr, Option::None);

    assert_eq!(result, exprected_result);
}

#[test]
fn it_visits_nested_math_expression() {
    let visitor = MockVisitor {};
    let math_expr = NumericalExpression {
        op: Operator::Add,
        node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
        node_b: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
            op: Operator::Multiply,
            node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 4 })),
        })),
    };

    let exprected_result = Result::Ok(14);

    let result = visitor.visit_math_expr(&math_expr, Option::None);

    assert_eq!(result, exprected_result);
}
