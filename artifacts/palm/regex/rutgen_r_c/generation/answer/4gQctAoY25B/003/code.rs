// Answer 0

#[test]
fn test_hir_literal_unicode_case_insensitive() {
    let literal = ast::Literal { 
        span: Span { start: Position(0), end: Position(1) }, 
        c: 'a' 
    };
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { case_insensitive: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "a");
    let result = translator_i.hir_literal(&literal);
    
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind().is_literal(), true);
}

#[test]
fn test_hir_literal_byte_case_insensitive() {
    let literal = ast::Literal { 
        span: Span { start: Position(0), end: Position(1) }, 
        c: 'b' 
    };
    
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { case_insensitive: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };

    let translator_i = TranslatorI::new(&translator, "b");
    let result = translator_i.hir_literal(&literal);
    
    assert!(result.is_ok());
    let hir = result.unwrap();
    assert_eq!(hir.kind().is_literal(), true);
}

