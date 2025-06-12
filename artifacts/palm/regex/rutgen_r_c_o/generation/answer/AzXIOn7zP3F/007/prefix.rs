// Answer 0

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_digit_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_space_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_word_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_digit_non_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_space_non_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_word_non_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    let translator_i = TranslatorI::new(&translator, "test_pattern");
    translator_i.hir_perl_unicode_class(&ast_class);
}

