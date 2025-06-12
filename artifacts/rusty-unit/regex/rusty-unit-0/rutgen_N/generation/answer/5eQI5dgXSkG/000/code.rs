// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct TestParser {
        input: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input
        }

        fn span_char(&self) -> usize {
            0 // Dummy span value
        }

        fn bump(&mut self) {
            // Simulate moving to the next character by resetting input (no-op for this test)
        }
    }

    let mut parser = TestParser { input: 'd' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert!(!result.negated);

    parser.input = 'D';
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Digit);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_space() {
    struct TestParser {
        input: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input
        }

        fn span_char(&self) -> usize {
            0 // Dummy span value
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let mut parser = TestParser { input: 's' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert!(!result.negated);

    parser.input = 'S';
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Space);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_word() {
    struct TestParser {
        input: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input
        }

        fn span_char(&self) -> usize {
            0 // Dummy span value
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let mut parser = TestParser { input: 'w' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert!(!result.negated);

    parser.input = 'W';
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ast::ClassPerlKind::Word);
    assert!(result.negated);
}

#[should_panic(expected = "expected valid Perl class but got 'x'")]
#[test]
fn test_parse_perl_class_invalid() {
    struct TestParser {
        input: char,
    }

    impl TestParser {
        fn char(&self) -> char {
            self.input
        }

        fn span_char(&self) -> usize {
            0 // Dummy span value
        }

        fn bump(&mut self) {
            // Simulate moving to the next character
        }
    }

    let mut parser = TestParser { input: 'x' };
    parser.parse_perl_class(); // This should panic
}

