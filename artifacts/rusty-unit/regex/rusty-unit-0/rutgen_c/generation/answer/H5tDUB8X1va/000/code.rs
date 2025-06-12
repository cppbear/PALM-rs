// Answer 0

#[test]
fn test_literal_to_char_unicode_mode() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
                allow_invalid_utf8: false,
            }
        }
    }

    let translator = TestTranslator::new();
    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "test",
    };

    let lit = ast::Literal { span: Span { start: Position(0), end: Position(1) }, kind: LiteralKind::Unicode, c: 'a' };
    let result = translator_i.literal_to_char(&lit).unwrap();
    assert_eq!(result, hir::Literal::Unicode('a'));
}

#[test]
fn test_literal_to_char_non_unicode_mode() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
                allow_invalid_utf8: false,
            }
        }
    }

    let translator = TestTranslator::new();
    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "test",
    };

    let lit = ast::Literal { span: Span { start: Position(0), end: Position(1) }, kind: LiteralKind::Byte, c: 'a' };
    let result = translator_i.literal_to_char(&lit).unwrap();
    assert_eq!(result, hir::Literal::Unicode('a'));

    let lit_invalid = ast::Literal { span: Span { start: Position(0), end: Position(1) }, kind: LiteralKind::Byte, c: 200 as char };
    let result_invalid = translator_i.literal_to_char(&lit_invalid);
    assert!(result_invalid.is_err());
}

#[test]
fn test_literal_to_char_invalid_utf8_allowed() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
                allow_invalid_utf8: true,
            }
        }
    }

    let translator = TestTranslator::new();
    let translator_i = TranslatorI {
        trans: &translator,
        pattern: "test",
    };

    let lit_invalid = ast::Literal { span: Span { start: Position(0), end: Position(1) }, kind: LiteralKind::Byte, c: 200 as char };
    let result_invalid = translator_i.literal_to_char(&lit_invalid).unwrap();
    assert_eq!(result_invalid, hir::Literal::Byte(200));
}

