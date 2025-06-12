// Answer 0

#[test]
fn test_hir_assertion_start_text() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { multi_line: Some(false), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::StartText };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartText));
}

#[test]
fn test_hir_assertion_start_line() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { multi_line: Some(true), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::StartLine };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::StartLine));
}

#[test]
fn test_hir_assertion_end_text() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { multi_line: Some(false), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::EndText };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndText));
}

#[test]
fn test_hir_assertion_end_line() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { multi_line: Some(true), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::EndLine };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::Anchor(hir::Anchor::EndLine));
}

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { unicode: Some(true), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::WordBoundary };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::Unicode));
}

#[test]
fn test_hir_assertion_word_boundary_ascii() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { unicode: Some(false), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::WordBoundary };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let result = translator_i.hir_assertion(&assertion);
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind(), &HirKind::WordBoundary(hir::WordBoundary::Ascii));
}

#[test]
#[should_panic]
fn test_hir_assertion_not_word_boundary_invalid_utf8() {
    struct MockTranslator {
        flags: Flags,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new(flags: Flags, allow_invalid_utf8: bool) -> Self {
            Self { flags, allow_invalid_utf8 }
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn allow_invalid_utf8(&self) -> bool {
            self.allow_invalid_utf8
        }
    }

    let translator = MockTranslator::new(Flags { unicode: Some(false), ..Flags::default() }, false);
    let assertion = ast::Assertion { span: Span { start: Position(0), end: Position(0) }, kind: ast::AssertionKind::NotWordBoundary };
    let translator_i = TranslatorI::new(&translator, "pattern");

    let _ = translator_i.hir_assertion(&assertion); // This should panic
}

