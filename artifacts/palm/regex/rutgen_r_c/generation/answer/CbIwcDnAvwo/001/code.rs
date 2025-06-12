// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_unicode() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::NotWordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let result = translator_i.hir_assertion(&assertion);
    
    assert!(result.is_ok());

    if let Ok(hir) = result {
        assert_eq!(hir.is_always_utf8(), true);
        assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
    }
}

#[test]
fn test_hir_assertion_not_word_boundary_ascii_invalid_utf8() {
    struct TestAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let assertion = TestAstAssertion {
        kind: ast::AssertionKind::NotWordBoundary,
        span: Span { start: Position(0), end: Position(1) },
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let result = translator_i.hir_assertion(&assertion);
    
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::InvalidUtf8);
    }
}

