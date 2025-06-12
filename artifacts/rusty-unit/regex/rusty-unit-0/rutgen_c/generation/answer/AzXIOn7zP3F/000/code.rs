// Answer 0

#[test]
fn test_hir_perl_unicode_class_digit() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }
    
    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

#[test]
fn test_hir_perl_unicode_class_space() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

#[test]
fn test_hir_perl_unicode_class_word() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

#[test]
fn test_hir_perl_unicode_class_digit_negated() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

#[test]
fn test_hir_perl_unicode_class_space_negated() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

#[test]
fn test_hir_perl_unicode_class_word_negated() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let pattern = "";
    let translator_instance = TranslatorI::new(&translator, pattern);
    
    let ast_class = ast::ClassPerl {
        span: Span::new(0, 0),
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    
    let class_unicode = translator_instance.hir_perl_unicode_class(&ast_class);
    assert!(class_unicode.ranges().len() > 0);
}

