// Answer 0

#[test]
fn test_hir_assertion_end_line_without_multiline() {
    struct MockAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }
    
    let assertion = MockAstAssertion {
        kind: ast::AssertionKind::EndLine,
        span: Span { start: 0, end: 1 },
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let translator_instance = TranslatorI::new(&translator, "pattern");

    let result = translator_instance.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir_result = result.unwrap();
    assert_eq!(hir_result.into_kind(), HirKind::Anchor(Anchor::EndText));
}

#[test]
fn test_hir_assertion_end_line_with_multiline() {
    struct MockAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let assertion = MockAstAssertion {
        kind: ast::AssertionKind::EndLine,
        span: Span { start: 0, end: 1 },
    };

    let mut flags = Flags::default();
    flags.multi_line = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_instance = TranslatorI::new(&translator, "pattern");

    let result = translator_instance.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir_result = result.unwrap();
    assert_eq!(hir_result.into_kind(), HirKind::Anchor(Anchor::EndLine));
}

#[test]
fn test_hir_assertion_start_line_without_multiline() {
    struct MockAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let assertion = MockAstAssertion {
        kind: ast::AssertionKind::StartLine,
        span: Span { start: 0, end: 1 },
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };

    let translator_instance = TranslatorI::new(&translator, "pattern");

    let result = translator_instance.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir_result = result.unwrap();
    assert_eq!(hir_result.into_kind(), HirKind::Anchor(Anchor::StartText));
}

#[test]
fn test_hir_assertion_start_line_with_multiline() {
    struct MockAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let assertion = MockAstAssertion {
        kind: ast::AssertionKind::StartLine,
        span: Span { start: 0, end: 1 },
    };

    let mut flags = Flags::default();
    flags.multi_line = Some(true);

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };

    let translator_instance = TranslatorI::new(&translator, "pattern");

    let result = translator_instance.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir_result = result.unwrap();
    assert_eq!(hir_result.into_kind(), HirKind::Anchor(Anchor::StartLine));
}

