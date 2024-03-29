#[cfg(test)]
mod visitor_tests {
    use crate::parser::expressions::{
        FunctionCall, Negation, NumberLiteral, NumericalExpression, Terenary, Variable,
    };
    use crate::parser::statements::{Assignment, FunctionDeclaration};
    use crate::parser::ExpressionNode;
    use crate::tokenizer::Operator;
    use crate::visitor::{Context, VisitExpressionResult, VisitStatementResult, Visitor};

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

    #[test]
    fn it_visits_negated_expressions() {
        let visitor = MockVisitor {};
        // - 2 + 3
        let math_expr = NumericalExpression {
            op: Operator::Add,
            node_a: Box::new(ExpressionNode::Negation(Negation {
                expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
            })),
            node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
        };

        let exprected_result = Result::Ok(1);

        let result = visitor.visit_math_expr(&math_expr, Option::None);

        assert_eq!(result, exprected_result);
    }

    #[test]
    fn it_visits_terenary() {
        let visitor = MockVisitor {};
        // "1 + 2 ? -3 : 4"
        let terenary = Terenary {
            condition: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                op: Operator::Add,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 2 })),
            })),
            value: Box::new(ExpressionNode::Negation(Negation {
                expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            })),
            alternative: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 4 })),
        };

        let exprected_result = Result::Ok(-3);

        let result = visitor.visit_terenary(&terenary, Option::None);

        assert_eq!(result, exprected_result);
    }

    #[test]
    fn it_visits_falsy_terenary() {
        let visitor = MockVisitor {};
        // "1 - 1 ? -3 : 4"
        let terenary = Terenary {
            condition: Box::new(ExpressionNode::NumericalExpression(NumericalExpression {
                op: Operator::Subtract,
                node_a: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
                node_b: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 1 })),
            })),
            value: Box::new(ExpressionNode::Negation(Negation {
                expression: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 3 })),
            })),
            alternative: Box::new(ExpressionNode::NumberLiteral(NumberLiteral { value: 4 })),
        };

        let exprected_result = Result::Ok(4);

        let result = visitor.visit_terenary(&terenary, Option::None);

        assert_eq!(result, exprected_result);
    }
}
