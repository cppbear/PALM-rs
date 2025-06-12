// Answer 0

#[test]
fn test_hir_assertion_start_line_non_multiline() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::StartLine,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_line_non_multiline() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 1, end: 2 },
        kind: ast::AssertionKind::EndLine,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_start_text() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 2, end: 3 },
        kind: ast::AssertionKind::StartText,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_text() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 3, end: 4 },
        kind: ast::AssertionKind::EndText,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 4, end: 5 },
        kind: ast::AssertionKind::WordBoundary,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_not_word_boundary() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };

    let assertion = ast::Assertion {
        span: Span { start: 5, end: 6 },
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_assertion(&assertion);
}

