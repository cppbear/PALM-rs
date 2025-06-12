// Answer 0

#[test]
fn test_hir_perl_unicode_class_space_negated() {
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    let translator = Translator {
        flags: Cell::new(flags),
        stack: RefCell::new(vec![]),
        allow_invalid_utf8: false,
    };
    
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let result = translator_i.hir_perl_unicode_class(&ast_class);
}

#[test]
fn test_hir_perl_unicode_class_space_non_negated() {
    let flags = Flags {
        unicode: Some(true),
        ..Flags::default()
    };
    let translator = Translator {
        flags: Cell::new(flags),
        stack: RefCell::new(vec![]),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let translator_i = TranslatorI::new(&translator, "");

    let result = translator_i.hir_perl_unicode_class(&ast_class);
}

