// Answer 0

#[test]
fn test_hir_perl_byte_class_digit_not_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let class_bytes = translator_i.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_digit_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let class_bytes = translator_i.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_space_not_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let class_bytes = translator_i.hir_perl_byte_class(&ast_class);
}

#[test]
fn test_hir_perl_byte_class_word_not_negated() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let translator_i = TranslatorI::new(&translator, "");
    let class_bytes = translator_i.hir_perl_byte_class(&ast_class);
}

