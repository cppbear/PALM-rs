// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    struct ParserBuilder {
        ignore_whitespace: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            Self { ignore_whitespace: false }
        }

        pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let result = parser.ignore_whitespace(true);
    assert!(result.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_disable() {
    struct ParserBuilder {
        ignore_whitespace: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            Self { ignore_whitespace: false }
        }

        pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let result = parser.ignore_whitespace(false);
    assert!(!result.ignore_whitespace);
}

#[test]
fn test_ignore_whitespace_no_effect_on_other_fields() {
    struct ParserBuilder {
        ignore_whitespace: bool,
        some_other_field: i32,
    }

    impl ParserBuilder {
        fn new() -> Self {
            Self { ignore_whitespace: false, some_other_field: 42 }
        }

        pub fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ignore_whitespace = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    assert_eq!(parser.some_other_field, 42);
    parser.ignore_whitespace(true);
    assert_eq!(parser.some_other_field, 42);
}

