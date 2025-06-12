// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character, would do nothing here
        }
    }

    let parser = TestParser { state: 'd' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 'D' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_space() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 's' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 'S' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_word() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 'w' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 'W' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(result.negated);
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'x'")]
fn test_parse_perl_class_invalid_character() {
    struct TestParser {
        state: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.state
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let parser = TestParser { state: 'x' };
    parser.parse_perl_class();
}

