// Answer 0

#[test]
fn test_hir_from_char_case_insensitive_unicode() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    
    let translator_i = TranslatorI::new(&trans, "a");
    let span = Span { start: Position(0), end: Position(1) };
    let result = translator_i.hir_from_char_case_insensitive(span, 'a');
    
    assert!(result.is_ok());
}

#[test]
fn test_hir_from_char_case_insensitive_non_unicode() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&trans, "a");
    let span = Span { start: Position(0), end: Position(1) };
    let result = translator_i.hir_from_char_case_insensitive(span, 'a');

    assert!(result.is_ok());
}

#[test]
fn test_hir_from_char_case_insensitive_unicode_not_allowed() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&trans, "𝓪"); // A multi-byte character
    let span = Span { start: Position(0), end: Position(1) };
    let result = translator_i.hir_from_char_case_insensitive(span, '𝓪');

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind, ErrorKind::UnicodeNotAllowed);
    }
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_folding() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&trans, "A"); 
    let span = Span { start: Position(0), end: Position(1) };
    let result = translator_i.hir_from_char_case_insensitive(span, 'A');

    assert!(result.is_ok());
}

