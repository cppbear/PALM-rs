// Answer 0

fn test_visit_post_repetition() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = regex_syntax::ast::Ast::Repetition(regex_syntax::ast::Repetition::new(
        regex_syntax::ast::RepetitionKind::ZeroOrMore,
        Box::new(regex_syntax::ast::Ast::Literal(regex_syntax::ast::Literal::new('a'))),
    ));

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 0);
}

fn test_visit_post_empty() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = regex_syntax::ast::Ast::Empty;

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

fn test_visit_post_literal() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn decrement_depth(&mut self) {
            self.depth = self.depth.saturating_sub(1);
        }
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = regex_syntax::ast::Ast::Literal(regex_syntax::ast::Literal::new('b'));

    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

