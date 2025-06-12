// Answer 0

#[test]
fn test_hir_from_char_unicode_not_allowed() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            DummyTranslator {
                flags: Cell::new(Flags {
                    unicode: Some(false),
                    ..Flags::default()
                }),
            }
        }
    }

    let translator = DummyTranslator::new();
    let translator_ref = &translator;
    let span = Span { start: 0, end: 1 };
    let input_char = 'á'; // 'á' has a UTF-8 length greater than 1

    let result = translator_ref.hir_from_char(span, input_char);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::UnicodeNotAllowed);
        assert_eq!(error.pattern, "á");
        assert_eq!(error.span, span);
    }
}

