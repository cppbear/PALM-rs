// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_unicode_valid_utf8() {
    let pattern = ".*";
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: Position::from(0), end: Position::from(2) },
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&trans, pattern);
    translator_instance.hir_assertion(&assertion).unwrap();
}

#[test]
fn test_hir_assertion_not_word_boundary_unicode_negate() {
    let pattern = ".*";
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            multi_line: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: Position::from(0), end: Position::from(2) },
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&trans, pattern);
    translator_instance.hir_assertion(&assertion).unwrap();
}

#[test]
#[should_panic]
fn test_hir_assertion_not_word_boundary_unicode_invalid_utf8() {
    let pattern = ".*";
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: Position::from(0), end: Position::from(2) },
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator_instance = TranslatorI::new(&trans, pattern);
    translator_instance.hir_assertion(&assertion).unwrap();
}

