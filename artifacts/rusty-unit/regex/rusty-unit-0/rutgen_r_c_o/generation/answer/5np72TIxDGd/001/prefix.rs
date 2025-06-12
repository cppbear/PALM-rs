// Answer 0

#[test]
fn test_hir_from_char_case_insensitive_unicode_range_1() {
    let flags = Flags { unicode: Some(true), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), ..Translator::default() };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let span = Span { start: 0.into(), end: 1.into() };
    let c = 'A';
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_unicode_range_2() {
    let flags = Flags { unicode: Some(true), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), ..Translator::default() };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let span = Span { start: 0.into(), end: 1.into() };
    let c = 'z';
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_unicode_range_3() {
    let flags = Flags { unicode: Some(true), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), ..Translator::default() };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let span = Span { start: 0.into(), end: 1.into() };
    let c = '你';
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_unicode_range_4() {
    let flags = Flags { unicode: Some(true), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), ..Translator::default() };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let span = Span { start: 0.into(), end: 1.into() };
    let c = '😊';
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_unicode_edge_1() {
    let flags = Flags { unicode: Some(true), ..Flags::default() };
    let translator = Translator { flags: Cell::new(flags), ..Translator::default() };
    let translator_i = TranslatorI::new(&translator, "pattern");
    let span = Span { start: 0.into(), end: 1.into() };
    let c = '\u{10FFFF}';
    translator_i.hir_from_char_case_insensitive(span, c);
}

