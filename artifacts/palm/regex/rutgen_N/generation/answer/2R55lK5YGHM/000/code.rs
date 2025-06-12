// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    struct ParserBuilder {
        ignore_whitespace: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                ignore_whitespace: false,
            }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.ignore_whitespace(true);
    assert!(parser.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_disable() {
    struct ParserBuilder {
        ignore_whitespace: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                ignore_whitespace: false,
            }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.ignore_whitespace(false);
    assert!(!parser.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_multiple_calls() {
    struct ParserBuilder {
        ignore_whitespace: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                ignore_whitespace: false,
            }
        }

        fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.ignore_whitespace(false);
    parser.ignore_whitespace(true);
    assert!(parser.ignore_whitespace);
}

