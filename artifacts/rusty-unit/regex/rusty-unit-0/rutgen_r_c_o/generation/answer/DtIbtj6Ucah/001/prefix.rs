// Answer 0

#[test]
fn test_valid_byte_literal() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: 'a', kind: ast::LiteralKind::Unicode };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "a");
    translator_instance.class_literal_byte(&literal);
}

#[test]
fn test_unicode_character_below_limit() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: 'A', kind: ast::LiteralKind::Unicode };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "A");
    translator_instance.class_literal_byte(&literal);
}

#[test]
fn test_unicode_character_above_limit() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: 'Ω', kind: ast::LiteralKind::Unicode };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "Ω");
    translator_instance.class_literal_byte(&literal);
}

#[test]
fn test_byte_character() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: 'b' as u8 as char, kind: ast::LiteralKind::Byte };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "b");
    translator_instance.class_literal_byte(&literal);
}

#[should_panic]
#[test]
fn test_invalid_unicode_character() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: '∞', kind: ast::LiteralKind::Unicode };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "∞");
    translator_instance.class_literal_byte(&literal);
}

#[test]
fn test_lowest_byte_value() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: '\0', kind: ast::LiteralKind::Byte };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "\0");
    translator_instance.class_literal_byte(&literal);
}

#[test]
fn test_highest_byte_value() {
    let span = Span { start: 0, end: 1 };
    let literal = ast::Literal { span, c: 'ÿ', kind: ast::LiteralKind::Unicode };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let translator_instance = TranslatorI::new(&translator, "ÿ");
    translator_instance.class_literal_byte(&literal);
}

