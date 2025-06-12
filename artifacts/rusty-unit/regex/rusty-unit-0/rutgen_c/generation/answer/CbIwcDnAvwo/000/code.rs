// Answer 0

#[test]
fn test_hir_assertion_start_line_multiline() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            TestTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let translator = TestTranslator::new(Flags { multi_line: Some(true), ..Default::default() }, false);
    let translator_instance = Translator { flags: translator.flags, allow_invalid_utf8: translator.allow_invalid_utf8, ..Default::default() };
    let translator_i = TranslatorI { trans: &translator_instance, pattern: ".*" };
    let assertion = ast::Assertion { kind: ast::AssertionKind::StartLine, span: Span { start: 0, end: 1 } };
    
    let result = translator_i.hir_assertion(&assertion).unwrap();
    
    assert_eq!(result.kind(), &HirKind::Anchor(Anchor::StartLine));
}

#[test]
fn test_hir_assertion_end_line_non_multiline() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            TestTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let translator = TestTranslator::new(Flags { multi_line: Some(false), ..Default::default() }, false);
    let translator_instance = Translator { flags: translator.flags, allow_invalid_utf8: translator.allow_invalid_utf8, ..Default::default() };
    let translator_i = TranslatorI { trans: &translator_instance, pattern: ".*" };
    let assertion = ast::Assertion { kind: ast::AssertionKind::EndLine, span: Span { start: 0, end: 1 } };
    
    let result = translator_i.hir_assertion(&assertion).unwrap();
    
    assert_eq!(result.kind(), &HirKind::Anchor(Anchor::EndText));
}

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            TestTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let translator = TestTranslator::new(Flags { unicode: Some(true), ..Default::default() }, false);
    let translator_instance = Translator { flags: translator.flags, allow_invalid_utf8: translator.allow_invalid_utf8, ..Default::default() };
    let translator_i = TranslatorI { trans: &translator_instance, pattern: ".*" };
    let assertion = ast::Assertion { kind: ast::AssertionKind::WordBoundary, span: Span { start: 0, end: 1 } };
    
    let result = translator_i.hir_assertion(&assertion).unwrap();
    
    assert_eq!(result.kind(), &HirKind::WordBoundary(WordBoundary::Unicode));
}

#[test]
fn test_hir_assertion_not_word_boundary_ascii_invalid_utf8() {
    struct TestTranslator {
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl TestTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            TestTranslator {
                flags: Cell::new(flags),
                allow_invalid_utf8,
            }
        }
    }

    let translator = TestTranslator::new(Flags { unicode: Some(false), ..Default::default() }, false);
    let translator_instance = Translator { flags: translator.flags, allow_invalid_utf8: translator.allow_invalid_utf8, ..Default::default() };
    let translator_i = TranslatorI { trans: &translator_instance, pattern: ".*" };
    let assertion = ast::Assertion { kind: ast::AssertionKind::NotWordBoundary, span: Span { start: 0, end: 1 } };
    
    let result = translator_i.hir_assertion(&assertion);
    
    assert!(result.is_err());
}

