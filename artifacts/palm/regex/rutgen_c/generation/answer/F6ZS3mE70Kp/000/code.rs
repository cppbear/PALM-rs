// Answer 0

#[test]
fn test_hir_unicode_class_valid_one_letter() {
    struct MockTranslator {
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = MockTranslator::new();
    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('L'),
    };

    let result = translator.hir_unicode_class(&ast_class);
    assert!(result.is_ok());
}

#[test]
fn test_hir_unicode_class_unicode_not_allowed() {
    struct MockTranslator {
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
            }
        }
    }

    let translator = MockTranslator::new();
    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('L'),
    };

    let result = translator.hir_unicode_class(&ast_class);
    assert!(result.is_err());
    if let Err(ErrorKind::UnicodeNotAllowed) = result.err().unwrap() {}
}

#[test]
fn test_hir_unicode_class_property_not_found() {
    struct MockTranslator {
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = MockTranslator::new();
    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::Named("UnknownProperty".to_string()),
    };

    let result = translator.hir_unicode_class(&ast_class);
    assert!(result.is_err());
    if let Err(ErrorKind::UnicodePropertyNotFound) = result.err().unwrap() {}
}

#[test]
fn test_hir_unicode_class_property_value_not_found() {
    struct MockTranslator {
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
            }
        }
    }

    let translator = MockTranslator::new();
    let ast_class = ast::ClassUnicode {
        span: Span { start: Position(0), end: Position(1) },
        negated: false,
        kind: ast::ClassUnicodeKind::NamedValue {
            name: "PropertyName".to_string(),
            value: "UnknownValue".to_string(),
        },
    };

    let result = translator.hir_unicode_class(&ast_class);
    assert!(result.is_err());
    if let Err(ErrorKind::UnicodePropertyValueNotFound) = result.err().unwrap() {}
}

