// Answer 0

#[test]
fn test_into_class_literal_assertion_error() {
    let span = Span { start: 0, end: 1 };
    let assertion = Assertion { span, kind: ast::AssertionKind::start };
    let primitive = Primitive::Assertion(assertion);
    
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser // Assume Parser is properly instantiated elsewhere
        }
    }
    
    let parser_instance = ParserI::new(TestParser, "test pattern");
    let result = primitive.into_class_literal(&parser_instance);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassRangeLiteral);
        assert_eq!(err.pattern, "test pattern");
        assert_eq!(err.span, span);
    }
}

#[test]
fn test_into_class_literal_dot_error() {
    let span = Span { start: 2, end: 3 };
    let dot = Dot(span);
    let primitive = Primitive::Dot(dot);
    
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser // Assume Parser is properly instantiated elsewhere
        }
    }
    
    let parser_instance = ParserI::new(TestParser, "test pattern");
    let result = primitive.into_class_literal(&parser_instance);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassRangeLiteral);
        assert_eq!(err.pattern, "test pattern");
        assert_eq!(err.span, span);
    }
}

#[test]
fn test_into_class_literal_unicode_error() {
    let span = Span { start: 4, end: 5 };
    let unicode_class = ClassUnicode { span, negated: false, kind: ast::ClassUnicodeKind::Letter };
    let primitive = Primitive::Unicode(unicode_class);
    
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser // Assume Parser is properly instantiated elsewhere
        }
    }
    
    let parser_instance = ParserI::new(TestParser, "test pattern");
    let result = primitive.into_class_literal(&parser_instance);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassRangeLiteral);
        assert_eq!(err.pattern, "test pattern");
        assert_eq!(err.span, span);
    }
}

#[test]
fn test_into_class_literal_perl_error() {
    let span = Span { start: 6, end: 7 };
    let perl_class = ClassPerl { span, kind: ast::ClassPerlKind::Digit, negated: false };
    let primitive = Primitive::Perl(perl_class);
    
    struct TestParser;
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser // Assume Parser is properly instantiated elsewhere
        }
    }
    
    let parser_instance = ParserI::new(TestParser, "test pattern");
    let result = primitive.into_class_literal(&parser_instance);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.kind, ast::ErrorKind::ClassRangeLiteral);
        assert_eq!(err.pattern, "test pattern");
        assert_eq!(err.span, span);
    }
}

