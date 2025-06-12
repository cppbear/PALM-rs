// Answer 0

fn test_hir_perl_unicode_class_digit() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags {
                    unicode: Some(true),
                    ..Flags::default()
                }),
            }
        }
    }

    let translator = TestTranslator::new();
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = translator.hir_perl_unicode_class(&ast_class);
    assert!(result.iter().count() > 0); // Assuming it should produce a non-empty class
}

fn test_hir_perl_unicode_class_digit_negated() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }

    impl TestTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags {
                    unicode: Some(true),
                    ..Flags::default()
                }),
            }
        }
    }

    let translator = TestTranslator::new();
    let ast_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    let mut class = translator.hir_perl_unicode_class(&ast_class);
    // Assert that class is negated (how this is determined would depend on implementation details)
    class.negate(); // Assuming negate changes the state of the class
    assert!(class.iter().count() == 0); // Assuming negating the digit class makes it empty
}

