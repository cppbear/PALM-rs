// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            // Simulate advancing to the next character after parsing
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'd' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'D' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_space() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 's' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'S' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert_eq!(result.negated, true);
}

#[test]
fn test_parse_perl_class_word() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'w' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, false);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'W' };
    let result = parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert_eq!(result.negated, true);
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'a'")]
fn test_parse_perl_class_invalid_char() {
    struct MockParser {
        current_char: char,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn span_char(&self) -> Span {
            Span { start: 0, end: 1 }
        }

        fn bump(&mut self) {
            self.current_char = 'a'; // Advance to a non-Perl class character
        }
    }

    let mut parser = MockParser { current_char: 'a' };
    parser.parse_perl_class(); // This should panic
}

