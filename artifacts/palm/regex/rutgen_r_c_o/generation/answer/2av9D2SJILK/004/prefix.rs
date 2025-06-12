// Answer 0

#[test]
fn test_hir_dot_invalid_utf8_allowed() {
    let span = Span { start: 0, end: 0 };
    let flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: Some(false),
        swap_greed: None,
        unicode: Some(false),
    };
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let result = translator_i.hir_dot(span);
}

#[test]
fn test_hir_dot_no_utf8_with_flag() {
    let span = Span { start: 0, end: 0 };
    let flags = Flags {
        case_insensitive: None,
        multi_line: None,
        dot_matches_new_line: Some(false),
        swap_greed: None,
        unicode: Some(false),
    };
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let result = translator_i.hir_dot(span);
}

