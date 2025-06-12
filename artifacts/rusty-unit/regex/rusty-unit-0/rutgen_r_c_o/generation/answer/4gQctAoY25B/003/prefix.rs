// Answer 0

#[test]
fn test_hir_literal_unicode_char_a() {
    let flags = Flags { case_insensitive: Some(true), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let span = Span { start: Position::default(), end: Position::default() };
    let lit = ast::Literal { span, c: 'a', byte: None };
    let translator_i = TranslatorI::new(&translator, "a");
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_unicode_char_a_with_diacritic() {
    let flags = Flags { case_insensitive: Some(true), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: false,
    };
    let span = Span { start: Position::default(), end: Position::default() };
    let lit = ast::Literal { span, c: 'ä', byte: None };
    let translator_i = TranslatorI::new(&translator, "ä");
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_byte_representation() {
    let flags = Flags { case_insensitive: Some(true), ..Flags::default() };
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(flags),
        allow_invalid_utf8: true,
    };
    let span = Span { start: Position::default(), end: Position::default() };
    let lit = ast::Literal { span, c: '1', byte: Some(1) };
    let translator_i = TranslatorI::new(&translator, "1");
    translator_i.hir_literal(&lit);
}

