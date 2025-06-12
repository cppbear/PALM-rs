// Answer 0

#[test]
fn test_hir_assertion_not_word_boundary_invalid_utf8() {
    let span = Span { start: 0, end: 1 };
    let asst = ast::Assertion {
        span,
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            multi_line: Some(false),
            dot_matches_new_line: Some(false),
            swap_greed: Some(false),
            unicode: Some(false),
        }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "",
    };

    let _result = translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_not_word_boundary_unicode() {
    let span = Span { start: 0, end: 1 };
    let asst = ast::Assertion {
        span,
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            multi_line: Some(false),
            dot_matches_new_line: Some(false),
            swap_greed: Some(false),
            unicode: Some(true),
        }),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "",
    };

    let _result = translator_i.hir_assertion(&asst);
}

