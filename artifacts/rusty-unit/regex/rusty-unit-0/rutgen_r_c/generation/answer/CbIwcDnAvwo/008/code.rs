// Answer 0

#[test]
fn test_hir_assertion_end_line_multi_line() {
    struct TestVisitor;

    // Create dummy data for testing
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::AssertionKind::EndLine,
    };
    
    let flags = Flags {
        multi_line: Some(true),
        ..Default::default()
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");

    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndLine));
    }
}

#[test]
fn test_hir_assertion_start_line_multi_line() {
    struct TestVisitor;

    // Create dummy data for testing
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::AssertionKind::StartLine,
    };
    
    let flags = Flags {
        multi_line: Some(true),
        ..Default::default()
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");

    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartLine));
    }
}

#[test]
fn test_hir_assertion_end_text_multi_line() {
    struct TestVisitor;

    // Create dummy data for testing
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::AssertionKind::EndText,
    };
    
    let flags = Flags {
        multi_line: Some(true),
        ..Default::default()
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");

    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndText));
    }
}

#[test]
fn test_hir_assertion_start_text() {
    struct TestVisitor;

    // Create dummy data for testing
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(1) },
        kind: ast::AssertionKind::StartText,
    };
    
    let flags = Flags {
        multi_line: Some(true), // can still be true; tests StartText
        ..Default::default()
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");

    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartText));
    }
}

