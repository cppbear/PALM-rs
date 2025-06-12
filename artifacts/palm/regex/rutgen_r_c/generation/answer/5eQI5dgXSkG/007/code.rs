// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct TestParser;
    
    impl TestParser {
        fn char(&self) -> char {
            'd'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\d",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            'D'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\D",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_perl_class_space() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            's'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\s",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            'S'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\S",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_perl_class_word() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            'w'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\w",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct TestParser;

    impl TestParser {
        fn char(&self) -> char {
            'W'
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&self) {}

        // This function is required because the trait ParserI expects Borrow<Parser>
        fn borrow(&self) -> &Self {
            self
        }
    }

    let parser = ParserI {
        parser: TestParser,
        pattern: r"\W",
    };

    let result = parser.parse_perl_class();
    let expected = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    assert_eq!(result, expected);
}

