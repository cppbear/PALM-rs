// Answer 0

#[test]
fn test_hir_literal_unicode() {
    struct MockTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let lit = ast::Literal { span, c: 'a' }; // a valid Unicode character

    let translator = MockTranslator::new(Flags { unicode: Some(true), ..Flags::default() }, true);
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_literal(&lit).unwrap();

    assert_eq!(result.kind(), &HirKind::Literal(hir::Literal::Unicode('a')));
}

#[test]
fn test_hir_literal_byte() {
    struct MockTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let lit = ast::Literal { span, c: 'b' }; // a single character being transformed into byte literal

    let translator = MockTranslator::new(Flags { unicode: Some(false), ..Flags::default() }, true); 
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_literal(&lit).unwrap();
    
    assert_eq!(result.kind(), &HirKind::Literal(hir::Literal::Byte(b'b')));
}

#[test]
fn test_hir_literal_invalid_utf8() {
    struct MockTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let lit = ast::Literal { span, c: '©' }; // Non-ASCII character

    let translator = MockTranslator::new(Flags { unicode: Some(false), ..Flags::default() }, false); 
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_literal(&lit);
    
    assert!(result.is_err());
}

#[test]
fn test_hir_literal_case_insensitive() {
    struct MockTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            MockTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let lit = ast::Literal { span, c: 'c' }; // Valid character

    let translator = MockTranslator::new(Flags { case_insensitive: Some(true), unicode: Some(true) }, true);
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_literal(&lit).unwrap();
    
    assert_eq!(result.kind(), &HirKind::Literal(hir::Literal::Unicode('c')));
}

