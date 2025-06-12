// Answer 0

#[test]
fn test_class_literal_byte_with_byte_literal() {
    struct TestTranslator;
    impl TestTranslator {
        fn allow_invalid_utf8(&self) -> bool {
            false
        }
    }
    
    let trans = TestTranslator {};
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    
    let ast_literal = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Byte,
        c: 'a' as char, // Example byte literal
    };
    
    let result = translator_i.class_literal_byte(&ast_literal);
    assert_eq!(result, Ok('a' as u8));
}

#[test]
fn test_class_literal_byte_with_unicode_literal() {
    struct TestTranslator;
    impl TestTranslator {
        fn allow_invalid_utf8(&self) -> bool {
            false
        }
    }
    
    let trans = TestTranslator {};
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    
    let ast_literal = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Unicode,
        c: 'A', // Example unicode literal within ASCII range
    };
    
    let result = translator_i.class_literal_byte(&ast_literal);
    assert_eq!(result, Ok('A' as u8));
}

#[test]
fn test_class_literal_byte_with_unicode_out_of_range() {
    struct TestTranslator;
    impl TestTranslator {
        fn allow_invalid_utf8(&self) -> bool {
            false
        }
    }
    
    let trans = TestTranslator {};
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    
    let ast_literal = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Unicode,
        c: 'é', // Example unicode literal outside ASCII range
    };
    
    let result = translator_i.class_literal_byte(&ast_literal);
    assert!(result.is_err()); // Expect an error due to UnicodeNotAllowed
    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::UnicodeNotAllowed);
    }
}

