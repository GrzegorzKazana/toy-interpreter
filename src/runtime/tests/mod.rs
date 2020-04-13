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

    let expected_result = Result::Ok(3);

    let _ = mock_interpreter.visit_assignment(&assignment);
    let result = mock_interpreter.visit_var(&retrived_variable, Option::None);

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

    let expected_result = Result::Ok(3);

    let _ = mock_interpreter.visit_fn_declaration(&declaration);
    let result = mock_interpreter.visit_fn_call(&call, Option::None);

    assert_eq!(result, expected_result);
}

#[test]
fn it_handles_function_nesting() {
    let mut mock_interpreter = Runtime::new();
    // fun sum(a, b) = a + b
    let declaration_a = FunctionDeclaration {
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
    // fun sub(a, b) = sum(a, 0 - b)
    let declaration_b = FunctionDeclaration {
        identifier: String::from("sub"),
        expression: ExpressionNode::FunctionCall(FunctionCall {
            identifier: String::from("sum"),
            arguments: vec![
                ExpressionNode::Variable(Variable {
                    identifier: String::from("a"),
                }),
                ExpressionNode::NumericalExpression(NumericalExpression {
                    op: Operator::Subtract,
                    node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 0 })),
                    node_b: Box::new(ExpressionNode::Variable(Variable {
                        identifier: String::from("b"),
                    })),
                }),
            ],
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
        identifier: String::from("sub"),
        arguments: vec![
            ExpressionNode::NumberLiteral(NumberLiteral { value: 1 }),
            ExpressionNode::NumberLiteral(NumberLiteral { value: 2 }),
        ],
    };

    let expected_result = Result::Ok(-1);

    let _ = mock_interpreter.visit_fn_declaration(&declaration_a);
    let _ = mock_interpreter.visit_fn_declaration(&declaration_b);
    let result = mock_interpreter.visit_fn_call(&call, Option::None);

    assert_eq!(result, expected_result);
}
