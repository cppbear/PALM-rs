// Answer 0

#[test]
fn test_visit_post_empty() {
    let span = Span { start: 0, end: 0 };
    let ast = Ast::Empty(span);
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_unicode_class_error() {
    let span = Span { start: 0, end: 1 };
    let class_unicode = ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind::NamedValue {
            name: "property_name".to_string(),
            value: "property_value".to_string(),
        },
    };
    let ast = Ast::Class(ast::Class::Unicode(class_unicode));
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let result = translator_i.visit_post(&ast);
    assert!(result.is_err());
}

#[test]
fn test_visit_post_perl_class() {
    let span = Span { start: 0, end: 5 };
    let perl_class = ClassPerl {
        span,
        kind: ClassPerlKind::Word,
        negated: false,
    };
    let ast = Ast::Class(ast::Class::Perl(perl_class));
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_bracketed_class_error() {
    let span = Span { start: 1, end: 2 };
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Normal,
    };
    let ast = Ast::Class(ast::Class::Bracketed(class_bracketed));
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let result = translator_i.visit_post(&ast);
    assert!(result.is_err());
}

#[test]
fn test_visit_post_unicode_class_negated() {
    let span = Span { start: 1, end: 2 };
    let class_unicode = ClassUnicode {
        span,
        negated: true,
        kind: ClassUnicodeKind::Named {
            name: "some_unicode_property".to_string(),
        },
    };
    let ast = Ast::Class(ast::Class::Unicode(class_unicode));
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let result = translator_i.visit_post(&ast);
    assert!(result.is_err());
}

