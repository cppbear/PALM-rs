// Answer 0

#[test]
fn test_visit_pre_empty() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Empty(());
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

#[test]
fn test_visit_pre_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Literal(Literal::new("test"));
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

#[test]
fn test_visit_pre_dot() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Dot(());
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

#[test]
fn test_visit_pre_class_bracketed() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Class(Class::Bracketed(BracketedClass { span: () }));
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_repetition() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Repetition(Repetition { span: () });
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_group() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Group(Group { span: () });
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_alternation() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Alternation(Alternation { span: () });
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_concat() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _: &()) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    let mut visitor = TestVisitor { depth: 0 };
    let ast = Ast::Concat(Concat { span: () });
    let result = visitor.visit_pre(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

