// Answer 0

#[test]
fn test_hir_literal_unicode() {
    struct MockAstLiteral {
        c: char,
        span: Span,
    }
    
    let lit = MockAstLiteral { c: 'a', span: Span { start: 0, end: 1 } };
    let flags = Flags { case_insensitive: Some(false), ..Flags::default() };
    let trans = Translator { stack: RefCell::new(vec![]), flags: Cell::new(flags), allow_invalid_utf8: true };
    let translator_i = TranslatorI::new(&trans, "a");
    
    let result = translator_i.hir_literal(&lit);
    assert!(result.is_ok());
}

#[test]
fn test_hir_literal_case_insensitive() {
    struct MockAstLiteral {
        c: char,
        span: Span,
    }
    
    let lit = MockAstLiteral { c: 'A', span: Span { start: 0, end: 1 } };
    let flags = Flags { case_insensitive: Some(true), ..Flags::default() };
    let trans = Translator { stack: RefCell::new(vec![]), flags: Cell::new(flags), allow_invalid_utf8: true };
    let translator_i = TranslatorI::new(&trans, "A");
    
    let result = translator_i.hir_literal(&lit);
    assert!(result.is_ok());
}

#[test]
fn test_hir_literal_invalid_utf8() {
    struct MockAstLiteral {
        c: char,
        span: Span,
    }
    
    let lit = MockAstLiteral { c: 'ÿ', span: Span { start: 0, end: 1 } };
    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let trans = Translator { stack: RefCell::new(vec![]), flags: Cell::new(flags), allow_invalid_utf8: false };
    let translator_i = TranslatorI::new(&trans, "ÿ");
    
    let result = translator_i.hir_literal(&lit);
    assert!(result.is_err());
}

#[test]
fn test_hir_literal_byte() {
    struct MockAstLiteral {
        c: char,
        span: Span,
    }
    
    let lit = MockAstLiteral { c: 'b', span: Span { start: 0, end: 1 } };
    let flags = Flags { unicode: Some(false), ..Flags::default() };
    let trans = Translator { stack: RefCell::new(vec![]), flags: Cell::new(flags), allow_invalid_utf8: true };
    let translator_i = TranslatorI::new(&trans, "b");
    
    let result = translator_i.hir_literal(&lit);
    assert!(result.is_ok());
}

