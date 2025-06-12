// Answer 0

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_0() {
    let span = Span { start: 0, end: 1 };
    let c = '\u{0000}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_max() {
    let span = Span { start: 1, end: 2 };
    let c = '\u{D7FF}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_min() {
    let span = Span { start: 2, end: 3 };
    let c = '\u{E000}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_end() {
    let span = Span { start: 3, end: 4 };
    let c = '\u{FFFF}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_over_max_unicode() {
    let span = Span { start: 4, end: 5 };
    let c = '\u{10000}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

#[test]
fn test_hir_from_char_case_insensitive_no_case_mapping_high() {
    let span = Span { start: 5, end: 6 };
    let c = '\u{10FFFF}';
    let translator = Translator::default();
    let translator_i = TranslatorI::new(&translator, "test pattern");
    translator_i.hir_from_char_case_insensitive(span, c);
}

