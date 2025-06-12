// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    struct MockAst {
        ignore_whitespace: bool,
    }

    struct ParserBuilder {
        ast: MockAst,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                ast: MockAst {
                    ignore_whitespace: false,
                },
            }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
            self.ast.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let result = parser.ignore_whitespace(true);

    assert_eq!(result.ast.ignore_whitespace, true);
}

#[test]
fn test_ignore_whitespace_disable() {
    struct MockAst {
        ignore_whitespace: bool,
    }

    struct ParserBuilder {
        ast: MockAst,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                ast: MockAst {
                    ignore_whitespace: true,
                },
            }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
            self.ast.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let result = parser.ignore_whitespace(false);

    assert_eq!(result.ast.ignore_whitespace, false);
}

