// Answer 0

fn test_visit_post_concat() {
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

    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
        Repetition(),
        Group(),
        Alternation(),
        Concat(Box<Ast>),
    }

    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(String),
    }

    let mut visitor = TestVisitor { depth: 1 };
    let ast = Ast::Concat(Box::new(Ast::Literal()));

    let result = visitor.visit_post(&ast);
    assert_eq!(result, Ok(()));
}

