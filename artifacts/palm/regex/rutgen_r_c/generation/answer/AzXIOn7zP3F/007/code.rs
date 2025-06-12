// Answer 0

fn test_hir_perl_unicode_class_digit() {
    struct DummyTranslator {
        flags: Flags,
    }

    impl DummyTranslator {
        fn new(flags: Flags) -> Self {
            DummyTranslator { flags }
        }

        fn flags(&self) -> Flags {
            self.flags.clone()
        }
    }

    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };

    let translator = DummyTranslator::new(flags);
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = std::panic::catch_unwind(|| {
        translator.hir_perl_unicode_class(&ast_class);
    });

    assert!(result.is_err());
}

fn test_hir_perl_unicode_class_space() {
    struct DummyTranslator {
        flags: Flags,
    }

    impl DummyTranslator {
        fn new(flags: Flags) -> Self {
            DummyTranslator { flags }
        }

        fn flags(&self) -> Flags {
            self.flags.clone()
        }
    }

    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };

    let translator = DummyTranslator::new(flags);
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let result = std::panic::catch_unwind(|| {
        translator.hir_perl_unicode_class(&ast_class);
    });

    assert!(result.is_err());
}

fn test_hir_perl_unicode_class_word() {
    struct DummyTranslator {
        flags: Flags,
    }

    impl DummyTranslator {
        fn new(flags: Flags) -> Self {
            DummyTranslator { flags }
        }

        fn flags(&self) -> Flags {
            self.flags.clone()
        }
    }

    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };

    let translator = DummyTranslator::new(flags);
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = std::panic::catch_unwind(|| {
        translator.hir_perl_unicode_class(&ast_class);
    });

    assert!(result.is_err());
}

fn test_hir_perl_unicode_class_negated() {
    struct DummyTranslator {
        flags: Flags,
    }

    impl DummyTranslator {
        fn new(flags: Flags) -> Self {
            DummyTranslator { flags }
        }

        fn flags(&self) -> Flags {
            self.flags.clone()
        }
    }

    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };

    let translator = DummyTranslator::new(flags);
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let result = std::panic::catch_unwind(|| {
        translator.hir_perl_unicode_class(&ast_class);
    });

    assert!(result.is_err());
}

