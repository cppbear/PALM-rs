// Answer 0

#[test]
fn test_hir_from_char_case_insensitive_unicode_not_allowed() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = '\u{10000}';

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let result = translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_non_ascii() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = '\u{10000}';

    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    let result = translator_i.hir_from_char_case_insensitive(span, c);
}

