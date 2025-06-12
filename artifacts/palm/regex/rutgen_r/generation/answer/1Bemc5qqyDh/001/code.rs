// Answer 0

#[test]
fn test_visit_empty_ast() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_empty(&mut self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::new_empty();
    let result = visit(&ast, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_simple_ast() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = usize;
        type Err = ();

        fn visit_literal(&mut self, _literal: &str) -> Result<Self::Output, Self::Err> {
            Ok(1)
        }
    }

    let ast = Ast::new_literal("a");
    let result = visit(&ast, TestVisitor);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_visit_complex_ast() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = usize;
        type Err = ();

        fn visit_concat(&mut self, left: &Ast, right: &Ast) -> Result<Self::Output, Self::Err> {
            let left_count = left.visit(self)?;
            let right_count = right.visit(self)?;
            Ok(left_count + right_count)
        }

        fn visit_literal(&mut self, _literal: &str) -> Result<Self::Output, Self::Err> {
            Ok(1)
        }
    }

    let left_ast = Ast::new_literal("a");
    let right_ast = Ast::new_literal("b");
    let ast = Ast::new_concat(left_ast, right_ast);
    let result = visit(&ast, TestVisitor);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_visit_error_handling() {
    struct ErrorVisitor;
    impl Visitor for ErrorVisitor {
        type Output = ();
        type Err = ();

        fn visit_literal(&mut self, _literal: &str) -> Result<Self::Output, Self::Err> {
            Err(())
        }
    }

    let ast = Ast::new_literal("a");
    let result = visit(&ast, ErrorVisitor);
    assert!(result.is_err());
}

