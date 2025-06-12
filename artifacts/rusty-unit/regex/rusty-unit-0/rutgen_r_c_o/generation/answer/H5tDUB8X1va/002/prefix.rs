// Answer 0

#[test]
fn test_literal_to_char_with_non_ascii_byte_and_invalid_utf8_disabled() {
    let translator = Translator { 
        stack: RefCell::new(Vec::new()), 
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }), 
        allow_invalid_utf8: false 
    };

    let span = Span { start: Position(0), end: Position(1) };
    let lit = ast::Literal { span, kind: LiteralKind::HexFixed(HexLiteralKind::X), c: '\u{007F}' };

    let translator_instance = TranslatorI::new(&translator, "test");
    translator_instance.literal_to_char(&lit);
}

#[test]
fn test_literal_to_char_with_ascii_byte() {
    let translator = Translator { 
        stack: RefCell::new(Vec::new()), 
        flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }), 
        allow_invalid_utf8: false 
    };

    let span = Span { start: Position(0), end: Position(1) };
    let lit = ast::Literal { span, kind: LiteralKind::HexFixed(HexLiteralKind::X), c: 'A' };

    let translator_instance = TranslatorI::new(&translator, "test");
    translator_instance.literal_to_char(&lit);
}

