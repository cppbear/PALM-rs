// Answer 0

#[test]
fn test_hir_assertion_start_line_multi_line_true() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(true),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::StartLine,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_line_multi_line_true() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(true),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::EndLine,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_start_text_multi_line_true() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(true),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::StartText,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_end_text_multi_line_true() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(true),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::EndText,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_unicode_false() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(false),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::WordBoundary,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_not_word_boundary_unicode_false() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(false),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: true,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

#[test]
fn test_hir_assertion_word_boundary_multi_line_false() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            multi_line: Some(false),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };
    let assertion = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::WordBoundary,
    };
    let translator_i = TranslatorI::new(&trans, "");
    let _ = translator_i.hir_assertion(&assertion);
}

