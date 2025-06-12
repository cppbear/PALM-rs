// Answer 0

#[test]
fn test_hir_assertion_start_line() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(false), unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::StartLine, span: Span { start: 0, end: 0 } };
    
    let result = translator_i.hir_assertion(&asst).unwrap();
    assert_eq!(result, Hir::anchor(hir::Anchor::StartText));
}

#[test]
fn test_hir_assertion_end_line() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { multi_line: Some(true), unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::EndLine, span: Span { start: 0, end: 0 } };
    
    let result = translator_i.hir_assertion(&asst).unwrap();
    assert_eq!(result, Hir::anchor(hir::Anchor::EndLine));
}

#[test]
fn test_hir_assertion_start_text() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::StartText, span: Span { start: 0, end: 0 } };
    
    let result = translator_i.hir_assertion(&asst).unwrap();
    assert_eq!(result, Hir::anchor(hir::Anchor::StartText));
}

#[test]
fn test_hir_assertion_end_text() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::EndText, span: Span { start: 0, end: 0 } };
    
    let result = translator_i.hir_assertion(&asst).unwrap();
    assert_eq!(result, Hir::anchor(hir::Anchor::EndText));
}

#[test]
fn test_hir_assertion_word_boundary() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: true,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::WordBoundary, span: Span { start: 0, end: 0 } };
    
    let result = translator_i.hir_assertion(&asst).unwrap();
    assert_eq!(result, Hir::word_boundary(hir::WordBoundary::Ascii));
}

#[test]
fn test_hir_assertion_not_word_boundary() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), allow_invalid_utf8: false, ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let pattern = "";
    let translator_i = TranslatorI::new(&trans, pattern);
    let asst = ast::Assertion { kind: ast::AssertionKind::NotWordBoundary, span: Span { start: 0, end: 0 } };
    
    let err = translator_i.hir_assertion(&asst).err().unwrap();
    assert_eq!(err.kind, ErrorKind::InvalidUtf8);
}

