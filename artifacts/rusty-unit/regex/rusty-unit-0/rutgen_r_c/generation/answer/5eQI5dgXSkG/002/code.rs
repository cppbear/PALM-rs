// Answer 0

#[test]
fn test_parse_perl_class_digit() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 'd',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(!result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

#[test]
fn test_parse_perl_class_negated_digit() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 'D',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Digit);
    assert!(result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

#[test]
fn test_parse_perl_class_space() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 's',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(!result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

#[test]
fn test_parse_perl_class_negated_space() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 'S',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Space);
    assert!(result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

#[test]
fn test_parse_perl_class_word() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 'w',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(!result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

#[test]
fn test_parse_perl_class_negated_word() {
    struct MockParser {
        char_return: char,
        span_return: Span,
    }

    impl MockParser {
        fn char(&self) -> char {
            self.char_return
        }
        
        fn span_char(&self) -> Span {
            self.span_return
        }

        fn bump(&self) {}
    }

    let mock = MockParser {
        char_return: 'W',
        span_return: Span { start: 0, end: 1 },
    };
    
    let result = parse_perl_class(&mock);
    assert_eq!(result.kind, ClassPerlKind::Word);
    assert!(result.negated);
    assert_eq!(result.span.start, 0);
    assert_eq!(result.span.end, 1);
}

