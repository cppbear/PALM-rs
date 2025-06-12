// Answer 0

#[test]
fn test_hir_assertion_end_line_no_multiline() {
    struct DummyAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let span = Span { start: Position::from(0), end: Position::from(0) };
    let assertion = DummyAstAssertion {
        kind: ast::AssertionKind::EndLine,
        span,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir = result.unwrap();

    if let Hir::Anchor(anchor) = hir.kind() {
        assert_eq!(*anchor, hir::Anchor::EndText);
    } else {
        panic!("Unexpected Hir kind: {:?}", hir.kind());
    }
}

#[test]
fn test_hir_assertion_start_line_no_multiline() {
    struct DummyAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let span = Span { start: Position::from(0), end: Position::from(0) };
    let assertion = DummyAstAssertion {
        kind: ast::AssertionKind::StartLine,
        span,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir = result.unwrap();

    if let Hir::Anchor(anchor) = hir.kind() {
        assert_eq!(*anchor, hir::Anchor::StartText);
    } else {
        panic!("Unexpected Hir kind: {:?}", hir.kind());
    }
} 

#[test]
fn test_hir_assertion_word_boundary_unicode() {
    struct DummyAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let span = Span { start: Position::from(0), end: Position::from(0) };
    let assertion = DummyAstAssertion {
        kind: ast::AssertionKind::WordBoundary,
        span,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: true,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_ok());
    let hir = result.unwrap();

    if let Hir::WordBoundary(word_boundary) = hir.kind() {
        assert_eq!(*word_boundary, hir::WordBoundary::Unicode);
    } else {
        panic!("Unexpected Hir kind: {:?}", hir.kind());
    }
}

#[test]
fn test_hir_assertion_not_word_boundary_ascii_negate() {
    struct DummyAstAssertion {
        kind: ast::AssertionKind,
        span: Span,
    }

    let span = Span { start: Position::from(0), end: Position::from(0) };
    let assertion = DummyAstAssertion {
        kind: ast::AssertionKind::NotWordBoundary,
        span,
    };

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), allow_invalid_utf8: false }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "test pattern");
    let result = translator_i.hir_assertion(&assertion);

    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::InvalidUtf8);
    }
}

