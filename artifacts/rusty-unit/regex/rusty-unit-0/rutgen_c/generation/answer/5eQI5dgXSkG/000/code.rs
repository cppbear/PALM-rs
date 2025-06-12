// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'd',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'D',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_space() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 's',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'S',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_word() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'w',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'W',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, true);
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'x'")]
fn test_parse_perl_class_invalid_character() {
    struct ParserMock {
        char_return: char,
        span_return: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.char_return
        }

        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let parser = ParserMock {
        char_return: 'x',
        span_return: Span { start: Position(0), end: Position(1) },
    };

    parser.parse_perl_class();
}

