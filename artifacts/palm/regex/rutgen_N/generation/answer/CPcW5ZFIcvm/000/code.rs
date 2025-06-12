// Answer 0

#[test]
fn test_visit_post_empty() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Empty(());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_post_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Literal("test".into());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_post_dot() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Dot(());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_post_class_universal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Class(ast::Class::Unicode(vec!['a', 'b', 'c']));
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_post_repetition() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Repetition(Box::new(Ast::Literal("test".into())), 1, 3);
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

#[test]
fn test_visit_post_group() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Group(Box::new(Ast::Literal("test".into())));
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

