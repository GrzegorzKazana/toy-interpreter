#[allow(unused_imports)]
use super::Runtime;
#[allow(unused_imports)]
use super::Visitor;
#[allow(unused_imports)]
use crate::parser::expressions::{
    ExpressionNode, FunctionCall, NumberLiteral, NumericalExpression, Variable,
};
#[allow(unused_imports)]
use crate::parser::statements::{Assignment, FunctionDeclaration};
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

#[test]
fn it_accepts_declared_function_and_handles_call() {
    let mut mock_interpreter = Runtime::new();
    let declaration = FunctionDeclaration {
        identifier: String::from("sum"),
        expression: ExpressionNode::NumericalExpression(NumericalExpression {
            op: Operator::Add,
            node_a: Box::new(ExpressionNode::Variable(Variable {
                identifier: String::from("a"),
            })),
            node_b: Box::new(ExpressionNode::Variable(Variable {
                identifier: String::from("b"),
            })),
        }),
        arguments: vec![
            Variable {
                identifier: String::from("a"),
            },
            Variable {
                identifier: String::from("b"),
            },
        ],
    };
    let call = FunctionCall {
        identifier: String::from("sum"),
        arguments: vec![
            ExpressionNode::NumberLiteral(NumberLiteral { value: 1 }),
            ExpressionNode::NumberLiteral(NumberLiteral { value: 2 }),
        ],
    };

    let expected_result = Option::Some(3);

    mock_interpreter.visit_fn_declaration(&declaration);
    let result = mock_interpreter.visit_fn_call(&call);

    assert_eq!(result, expected_result);
}
