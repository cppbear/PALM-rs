// Answer 0

#[test]
fn test_hir_from_char_unicode() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");

    let span = Span { start: Position(0), end: Position(1) };
    let character = 'ç'; // A valid unicode character

    let result = translator_i.hir_from_char(span, character);
    assert!(result.is_ok());

    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Literal(hir::Literal::Unicode(character)));
    }
}

#[test]
fn test_hir_from_char_valid_unicode() {
    let translator = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let translator_i = TranslatorI::new(&translator, "test");

    let span = Span { start: Position(0), end: Position(1) };
    let character = '𝓐'; // A valid multi-byte unicode character

    let result = translator_i.hir_from_char(span, character);
    assert!(result.is_ok());

    if let Ok(hir) = result {
        assert_eq!(hir.kind(), &HirKind::Literal(hir::Literal::Unicode(character)));
    }
}

