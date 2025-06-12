// Answer 0

#[test]
fn test_hir_from_char_unicode_not_allowed() {
    let span = Span { start: Position(0), end: Position(1) };
    let non_ascii_char = 'é'; // UTF-8 length is greater than 1
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_from_char(span, non_ascii_char);
}

#[test]
fn test_hir_from_char_unicode_not_allowed_emoji() {
    let span = Span { start: Position(0), end: Position(1) };
    let emoji_char = '🌍'; // UTF-8 length is greater than 1
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_from_char(span, emoji_char);
}

#[test]
fn test_hir_from_char_unicode_not_allowed_combined_char() {
    let span = Span { start: Position(0), end: Position(1) };
    let combined_char = '𝄞'; // UTF-8 length is greater than 1
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test pattern");
    let _ = translator_i.hir_from_char(span, combined_char);
}

