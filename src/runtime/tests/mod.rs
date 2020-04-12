#[allow(unused_imports)]
use super::Runtime;
#[allow(unused_imports)]
use super::Visitor;
#[allow(unused_imports)]
use crate::parser::expressions::{ExpressionNode, NumberLiteral, NumericalExpression, Variable};
#[allow(unused_imports)]
use crate::parser::statements::Assignment;
#[allow(unused_imports)]
use crate::tokenizer::Operator;

#[test]
fn it_stores_and_retirves_variables() {
    let mut mock_interpreter = Runtime::new();
    let assignment = Assignment {
        identifier: String::from("a"),
        expression: ExpressionNode::NumericalExpression(NumericalExpression {
            op: Operator::Add,
            node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
            node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
        }),
    };
    let retrived_variable = Variable {
        identifier: String::from("a"),
    };

    let expected_result = Option::Some(3);

    mock_interpreter.visit_assignment(&assignment);
    let result = mock_interpreter.visit_var(&retrived_variable);

    assert_eq!(result, expected_result);
}
