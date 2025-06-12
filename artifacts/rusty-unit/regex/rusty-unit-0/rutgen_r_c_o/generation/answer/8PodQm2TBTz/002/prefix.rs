// Answer 0

#[test]
fn test_bytes_fold_and_negate() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut class = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0x7F }]); // ASCII range
    let flags = Flags { case_insensitive: Some(true), ..Default::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    translator_i.bytes_fold_and_negate(&span, true, &mut class);
}

#[test]
fn test_bytes_fold_and_negate_empty_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut class = ClassBytes::empty(); // Empty class edge case
    let flags = Flags { case_insensitive: Some(true), ..Default::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    translator_i.bytes_fold_and_negate(&span, true, &mut class);
}

#[test]
fn test_bytes_fold_and_negate_no_case_folding() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut class = ClassBytes::new(vec![ClassBytesRange { start: 0x61, end: 0x7A }]); // a-z
    let flags = Flags { case_insensitive: Some(false), ..Default::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    translator_i.bytes_fold_and_negate(&span, true, &mut class);
}

#[test]
fn test_bytes_fold_and_negate_allow_invalid_utf8() {
    let span = Span { start: Position(0), end: Position(1) };
    let mut class = ClassBytes::new(vec![ClassBytesRange { start: 0x00, end: 0x7F }]); // ASCII range
    let flags = Flags { case_insensitive: Some(true), ..Default::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");

    translator_i.bytes_fold_and_negate(&span, true, &mut class);
}

