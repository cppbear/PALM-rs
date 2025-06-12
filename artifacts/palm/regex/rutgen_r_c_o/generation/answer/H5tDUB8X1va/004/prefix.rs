// Answer 0

#[test]
fn test_literal_to_char_invalid_utf8() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let span = Span { start: Position(0), end: Position(1) };
    let lit = ast::Literal { span: span, kind: ast::LiteralKind::Byte, c: 'ÿ' }; // byte 255

    let translator_i = TranslatorI::new(&translator, "test_pattern");
    let result = translator_i.literal_to_char(&lit);
}

#[test]
fn test_literal_to_char_invalid_utf8_high_byte() {
    let mut translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let span = Span { start: Position(0), end: Position(1) };
    let lit = ast::Literal { span: span, kind: ast::LiteralKind::Byte, c: '€' }; // byte 128

    let translator_i = TranslatorI::new(&translator, "test_pattern");
    let result = translator_i.literal_to_char(&lit);
}

