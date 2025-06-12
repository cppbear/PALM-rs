// Answer 0

#[test]
fn test_visit_pre_group() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
        Repetition(Box<Ast>),
        Group(Box<Ast>),
        Alternation(Box<Ast>),
        Concat(Box<Ast>),
    }

    enum Class {
        Unicode(),
        Perl(),
        Bracketed(BracketedClass),
    }

    struct BracketedClass {
        span: String,
    }

    type Result<T> = std::result::Result<T, ()>;

    let ast_group = Ast::Group(Box::new(Ast::Empty()));
    let mut visitor = TestVisitor { depth: 0 };

    let result = visitor.visit_pre(&ast_group);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

#[test]
fn test_visit_pre_repetition() {
    struct TestVisitor {
        depth: usize,
    }

    impl TestVisitor {
        fn increment_depth(&mut self, _span: &str) -> Result<()> {
            self.depth += 1;
            Ok(())
        }
    }

    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
        Repetition(Box<Ast>),
        Group(Box<Ast>),
        Alternation(Box<Ast>),
        Concat(Box<Ast>),
    }

    enum Class {
        Unicode(),
        Perl(),
        Bracketed(BracketedClass),
    }

    struct BracketedClass {
        span: String,
    }

    type Result<T> = std::result::Result<T, ()>;

    let ast_repetition = Ast::Repetition(Box::new(Ast::Group(Box::new(Ast::Empty()))));
    let mut visitor = TestVisitor { depth: 0 };

    let result = visitor.visit_pre(&ast_repetition);
    assert!(result.is_ok());
    assert_eq!(visitor.depth, 1);
}

