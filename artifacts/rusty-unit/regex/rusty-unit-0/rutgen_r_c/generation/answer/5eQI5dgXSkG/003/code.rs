// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'd',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'D',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_space() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 's',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'S',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(result.negated);
}

#[test]
fn test_parse_perl_class_word() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'w',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(!result.negated);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'W',
        current_span: Span { start: 0, end: 1 },
    };
    
    let result = mock_parser.parse_perl_class();
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(result.negated);
}

#[test]
#[should_panic(expected = "expected valid Perl class but got 'x'")]
fn test_parse_perl_class_invalid_character() {
    struct ParserMock {
        current_char: char,
        current_span: Span,
    }

    impl ParserMock {
        fn char(&self) -> char {
            self.current_char
        }
        
        fn span_char(&self) -> Span {
            self.current_span
        }
        
        fn bump(&self) {
            // Mock implementation to simulate advancing the parser position
        }
    }

    let mock_parser = ParserMock {
        current_char: 'x',
        current_span: Span { start: 0, end: 1 },
    };
    
    mock_parser.parse_perl_class(); // This should panic
}

