// Answer 0

#[test]
fn test_visit_pre_with_alternation() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let alt_ast = Ast::Alternation(vec![
        Ast::Literal("a".to_string()),
        Ast::Literal("b".to_string()),
    ]);

    let result = visitor.visit_pre(&alt_ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_with_multiple_alternations() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let alt_ast = Ast::Alternation(vec![
        Ast::Class(ast::Class::Bracketed(vec![Ast::Literal("c".to_string())])),
        Ast::Group(Box::new(Ast::Literal("d".to_string()))),
    ]);

    let result = visitor.visit_pre(&alt_ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

