// Answer 0

#[test]
fn test_hir_assertion_start_line() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            multi_line: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::StartLine,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::Anchor(hir::Anchor::StartLine));
}

#[test]
fn test_hir_assertion_end_line() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            multi_line: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::EndLine,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::Anchor(hir::Anchor::EndLine));
}

#[test]
fn test_hir_assertion_start_text() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            multi_line: None,
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::StartText,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::Anchor(hir::Anchor::StartText));
}

#[test]
fn test_hir_assertion_end_text() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            multi_line: None,
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::EndText,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::Anchor(hir::Anchor::EndText));
}

#[test]
fn test_hir_assertion_word_boundary() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::WordBoundary,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::WordBoundary(hir::WordBoundary::Unicode));
}

#[test]
fn test_hir_assertion_not_word_boundary() {
    let pattern = "abc";
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&trans, pattern);
    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let result = translator_i.hir_assertion(&assertion).unwrap();
    assert_eq!(result.kind(), &HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate));
}

