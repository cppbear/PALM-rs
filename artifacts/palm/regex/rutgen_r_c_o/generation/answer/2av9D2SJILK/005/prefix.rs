// Answer 0

#[test]
fn test_hir_dot_invalid_utf8() {
    let span = Span { start: 0, end: 0 };
    
    let flags = Flags {
        unicode: Some(false),
        dot_matches_new_line: Some(false),
        ..Flags::default()
    };

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, ".*");

    let result = translator_i.hir_dot(span);
}

#[test]
fn test_hir_dot_invalid_utf8_with_new_line() {
    let span = Span { start: 0, end: 0 };
    
    let flags = Flags {
        unicode: Some(false),
        dot_matches_new_line: Some(true),
        ..Flags::default()
    };

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, ".*");

    let result = translator_i.hir_dot(span);
}

