// Answer 0

#[test]
fn test_hir_assertion_end_line_multi_line() {
    let span = Span { start: 0, end: 1 };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::EndLine,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(true), ..Default::default() }),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_line_multi_line_unicode() {
    let span = Span { start: 0, end: 1 };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::EndLine,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(true), unicode: Some(true), ..Default::default() }),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_line_not_unicode() {
    let span = Span { start: 0, end: 1 };
    let assertion = ast::Assertion {
        span,
        kind: ast::AssertionKind::EndLine,
    };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(true), unicode: Some(false), ..Default::default() }),
        allow_invalid_utf8: true,
    };
    let translator_i = TranslatorI::new(&translator, "pattern");
    translator_i.hir_assertion(&assertion);
}

