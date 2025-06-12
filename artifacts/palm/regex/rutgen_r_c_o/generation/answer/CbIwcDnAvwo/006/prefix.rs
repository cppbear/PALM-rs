// Answer 0

#[test]
fn test_hir_assertion_start_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::StartLine,
    };
    translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_end_line() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::EndLine,
    };
    translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_start_text() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::StartText,
    };
    translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_end_text() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::EndText,
    };
    translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_word_boundary() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::WordBoundary,
    };
    translator_i.hir_assertion(&asst);
}

#[test]
fn test_hir_assertion_not_word_boundary() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&translator, pattern);
    let asst = ast::Assertion {
        span: Span { start: Position(0), end: Position(0) },
        kind: ast::AssertionKind::NotWordBoundary,
    };
    translator_i.hir_assertion(&asst);
}

