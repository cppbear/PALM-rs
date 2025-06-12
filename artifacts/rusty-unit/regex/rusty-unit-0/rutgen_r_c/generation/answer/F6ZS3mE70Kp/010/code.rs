// Answer 0

#[test]
fn test_hir_unicode_class_unicode_not_allowed() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }

    impl TranslatorI<'_, '_> {
        fn new(trans: &TestTranslator, pattern: &str) -> TranslatorI {
            TranslatorI { trans, pattern }
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error {
                kind,
                pattern: self.pattern.to_string(),
                span,
            }
        }

        fn flags(&self) -> Flags {
            self.trans.flags.get()
        }

        fn unicode_fold_and_negate(&self, _negated: bool, _class: &mut hir::ClassUnicode) {}
    }

    let flags = Flags {
        unicode: Some(false),
        ..Flags::default()
    };

    let translator = TestTranslator {
        flags: Cell::new(flags),
    };

    let ast_class = ast::ClassUnicode {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        negated: false,
        kind: ast::ClassUnicodeKind::OneLetter('a'),
    };

    let translator_instance = TranslatorI::new(&translator, "pattern");
    
    let result = translator_instance.hir_unicode_class(&ast_class);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::UnicodeNotAllowed);
}

