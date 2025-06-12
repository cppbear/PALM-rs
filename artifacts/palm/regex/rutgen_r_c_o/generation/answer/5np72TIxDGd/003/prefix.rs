// Answer 0

#[test]
fn test_hir_from_char_case_insensitive_lowercase() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = 'a';
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let _ = translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_uppercase() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = 'A';
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let _ = translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_specific_char() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = 'b';
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let _ = translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_punctuation() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = '!'; // testing with punctuation
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let _ = translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_digit() {
    let span = Span { start: Position(0), end: Position(1) };
    let c = '1'; // testing with a digit
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let _ = translator_i.hir_from_char_case_insensitive(span, c);
}

