// Answer 0

#[test]
fn test_literal_to_char_valid_unicode_scalar() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let span = Span {
        start: Position::new(0),
        end: Position::new(1),
    };

    let lit = ast::Literal {
        span,
        kind: ast::LiteralKind::Unicode,
        c: 'a',
    };

    let translator_i = TranslatorI::new(&translator, "a");
    translator_i.literal_to_char(&lit);
}

#[test]
fn test_literal_to_char_another_unicode_scalar() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let span = Span {
        start: Position::new(0),
        end: Position::new(1),
    };

    let lit = ast::Literal {
        span,
        kind: ast::LiteralKind::Unicode,
        c: 'µ',
    };

    let translator_i = TranslatorI::new(&translator, "µ");
    translator_i.literal_to_char(&lit);
}

#[test]
fn test_literal_to_char_non_ascii_unicode_scalar() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let span = Span {
        start: Position::new(0),
        end: Position::new(1),
    };

    let lit = ast::Literal {
        span,
        kind: ast::LiteralKind::Unicode,
        c: '中',
    };

    let translator_i = TranslatorI::new(&translator, "中");
    translator_i.literal_to_char(&lit);
}

