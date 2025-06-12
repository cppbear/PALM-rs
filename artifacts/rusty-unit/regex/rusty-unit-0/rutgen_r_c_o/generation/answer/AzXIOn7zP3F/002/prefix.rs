// Answer 0

#[test]
fn test_hir_perl_unicode_class_word_non_negated() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = translator.hir_perl_unicode_class(&ast_class);
}

#[test]
fn test_hir_perl_unicode_class_digit_non_negated() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = translator.hir_perl_unicode_class(&ast_class);
}

#[test]
fn test_hir_perl_unicode_class_space_non_negated() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let result = translator.hir_perl_unicode_class(&ast_class);
}

