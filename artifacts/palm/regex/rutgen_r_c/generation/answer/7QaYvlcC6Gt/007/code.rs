// Answer 0

#[test]
fn test_hir_perl_byte_class_digit_not_negated() {
    struct MockTranslator {
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
            }
        }
        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    struct MockTranslatorI<'t> {
        trans: &'t MockTranslator,
    }

    impl<'t> TranslatorI<'t, 'static> {
        fn new(trans: &'t MockTranslator) -> Self {
            TranslatorI { trans, pattern: "" }
        }
    }

    let translator = MockTranslator::new();
    let translator_i = MockTranslatorI::new(&translator);
    
    let ast_class = ast::ClassPerl {
        span: Span { start: 0, end: 1 }, // Assuming this struct requires a span
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = translator_i.hir_perl_byte_class(&ast_class);
    let expected_class = hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit);
    assert_eq!(result, expected_class);
}

