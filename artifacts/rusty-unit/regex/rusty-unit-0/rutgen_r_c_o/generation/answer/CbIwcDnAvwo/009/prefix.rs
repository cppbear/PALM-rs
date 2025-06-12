// Answer 0

#[test]
fn test_hir_assertion_end_line_not_multi_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: true,
    };

    let span = Span { start: Position(0), end: Position(0) };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::EndLine,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let _result = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_start_text_not_multi_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: true,
    };

    let span = Span { start: Position(0), end: Position(0) };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::StartText,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let _result = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_start_line_not_multi_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: true,
    };

    let span = Span { start: Position(0), end: Position(0) };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::StartLine,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let _result = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_text_not_multi_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: None,
            multi_line: Some(false),
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: None,
        }),
        allow_invalid_utf8: true,
    };

    let span = Span { start: Position(0), end: Position(0) };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::EndText,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let _result = translator_i.hir_assertion(&assertion);
}

