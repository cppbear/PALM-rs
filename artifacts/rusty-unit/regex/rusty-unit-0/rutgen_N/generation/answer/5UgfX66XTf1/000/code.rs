// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    struct ParserBuilder {
        ast: Ast,
    }

    struct Ast {
        ignore_whitespace: bool,
    }

    impl Ast {
        fn new() -> Self {
            Ast { ignore_whitespace: false }
        }

        fn ignore_whitespace(&mut self, yes: bool) {
            self.ignore_whitespace = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { ast: Ast::new() }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ast.ignore_whitespace(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.ignore_whitespace(true);
    assert_eq!(parser.ast.ignore_whitespace, true);
}

#[test]
fn test_ignore_whitespace_disable() {
    struct ParserBuilder {
        ast: Ast,
    }

    struct Ast {
        ignore_whitespace: bool,
    }

    impl Ast {
        fn new() -> Self {
            Ast { ignore_whitespace: false }
        }

        fn ignore_whitespace(&mut self, yes: bool) {
            self.ignore_whitespace = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { ast: Ast::new() }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ast.ignore_whitespace(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.ignore_whitespace(false);
    assert_eq!(parser.ast.ignore_whitespace, false);
}

