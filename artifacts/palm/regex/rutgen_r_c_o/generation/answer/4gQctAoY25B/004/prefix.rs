// Answer 0

#[test]
fn test_hir_literal_unicode() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, c: 'a' };
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_unicode_edge_case() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let lit = ast::Literal { span: Span { start: 0, end: 1 }, c: '\u{007F}' }; // max char in ASCII
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_non_ascii_char() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let lit = ast::Literal { span: Span { start: 1, end: 2 }, c: 'β' }; // non-ASCII character
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_low_byte() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let lit = ast::Literal { span: Span { start: 2, end: 3 }, c: '1' }; // numeric character
    translator_i.hir_literal(&lit);
}

#[test]
fn test_hir_literal_start_boundary() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };
    let pattern = "test";
    let translator_i = TranslatorI::new(&trans, pattern);
    let lit = ast::Literal { span: Span { start: 0, end: 10 }, c: 'A' }; // span larger than needed
    translator_i.hir_literal(&lit);
}

