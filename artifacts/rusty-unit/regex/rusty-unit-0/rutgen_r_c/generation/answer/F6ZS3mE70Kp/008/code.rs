// Answer 0

fn main() {
    // Test for when unicode is enabled, and the query is valid
    #[test]
    fn test_hir_unicode_class_valid() {
        struct AstClassUnicode {
            kind: ast::ClassUnicodeKind,
            span: Span,
            negated: bool,
        }

        let span = Span { start: Position(0), end: Position(1) };
        let class_unicode = AstClassUnicode {
            kind: ast::ClassUnicodeKind::OneLetter('a'),
            span,
            negated: false,
        };

        let translator = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                unicode: Some(true),
                ..Flags::default()
            }),
            allow_invalid_utf8: false,
        };

        let result = translator.hir_unicode_class(&class_unicode);
        assert!(result.is_ok());
    }

    // Test for when unicode is enabled but property is not found
    #[test]
    fn test_hir_unicode_class_property_not_found() {
        struct AstClassUnicode {
            kind: ast::ClassUnicodeKind,
            span: Span,
            negated: bool,
        }

        let span = Span { start: Position(0), end: Position(1) };
        let class_unicode = AstClassUnicode {
            kind: ast::ClassUnicodeKind::Named("InvalidProperty".to_string()),
            span,
            negated: false,
        };

        let translator = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                unicode: Some(true),
                ..Flags::default()
            }),
            allow_invalid_utf8: false,
        };

        let result = translator.hir_unicode_class(&class_unicode);
        assert_eq!(result, Err(translator.error(span, ErrorKind::UnicodePropertyNotFound)));
    }

    // Test for when unicode is enabled but property value is not found
    #[test]
    fn test_hir_unicode_class_property_value_not_found() {
        struct AstClassUnicode {
            kind: ast::ClassUnicodeKind,
            span: Span,
            negated: bool,
        }

        let span = Span { start: Position(0), end: Position(1) };
        let class_unicode = AstClassUnicode {
            kind: ast::ClassUnicodeKind::NamedValue {
                name: "PropertyName".to_string(),
                value: "InvalidValue".to_string(),
            },
            span,
            negated: false,
        };

        let translator = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                unicode: Some(true),
                ..Flags::default()
            }),
            allow_invalid_utf8: false,
        };

        let result = translator.hir_unicode_class(&class_unicode);
        assert_eq!(result, Err(translator.error(span, ErrorKind::UnicodePropertyValueNotFound)));
    }

    // Test when unicode is disabled
    #[test]
    fn test_hir_unicode_class_unicode_not_allowed() {
        struct AstClassUnicode {
            kind: ast::ClassUnicodeKind,
            span: Span,
            negated: bool,
        }

        let span = Span { start: Position(0), end: Position(1) };
        let class_unicode = AstClassUnicode {
            kind: ast::ClassUnicodeKind::OneLetter('a'),
            span,
            negated: false,
        };

        let translator = Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(Flags {
                unicode: Some(false),
                ..Flags::default()
            }),
            allow_invalid_utf8: false,
        };

        let result = translator.hir_unicode_class(&class_unicode);
        assert_eq!(result, Err(translator.error(span, ErrorKind::UnicodeNotAllowed)));
    }
}

