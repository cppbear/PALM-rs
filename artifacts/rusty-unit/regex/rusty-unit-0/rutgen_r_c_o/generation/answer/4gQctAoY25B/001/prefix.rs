// Answer 0

#[test]
fn test_hir_literal_null_character() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "\u{0000}";
    let literal = ast::Literal { span: Span { start: Position::default(), end: Position::default() }, c: '\u{0000}' };
    let translator_instance = TranslatorI::new(&translator, pattern);
    let _ = translator_instance.hir_literal(&literal);
}

#[test]
fn test_hir_literal_max_unicode_character() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "\u{10FFFF}";
    let literal = ast::Literal { span: Span { start: Position::default(), end: Position::default() }, c: '\u{10FFFF}' };
    let translator_instance = TranslatorI::new(&translator, pattern);
    let _ = translator_instance.hir_literal(&literal);
}

#[test]
fn test_hir_literal_ascii_character() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "A";
    let literal = ast::Literal { span: Span { start: Position::default(), end: Position::default() }, c: 'A' };
    let translator_instance = TranslatorI::new(&translator, pattern);
    let _ = translator_instance.hir_literal(&literal);
}

#[test]
fn test_hir_literal_non_ascii_character_non_unicode() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "ç";
    let literal = ast::Literal { span: Span { start: Position::default(), end: Position::default() }, c: 'ç' };
    let translator_instance = TranslatorI::new(&translator, pattern);
    let _ = translator_instance.hir_literal(&literal);
}

#[test]
fn test_hir_literal_invalid_byte_for_utf8() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let pattern = "𐍈";
    let literal = ast::Literal { span: Span { start: Position::default(), end: Position::default() }, c: '𐍈' };
    let translator_instance = TranslatorI::new(&translator, pattern);
    let _ = translator_instance.hir_literal(&literal);
}

