// Answer 0

fn test_visit_class_set_item_post_perl_unicode() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(Vec::new()),
                flags: Cell::new(Flags {
                    unicode: Some(true),
                    ..Flags::default()
                }),
                allow_invalid_utf8: true,
            }
        }
    }

    let mut translator = MockTranslator::new();

    let perl_class = ast::ClassPerl {
        span: Span {
            start: Position(0),
            end: Position(1),
        },
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let class_set_item = ast::ClassSetItem::Perl(perl_class);

    translator.stack.borrow_mut().push(HirFrame::ClassUnicode(hir::ClassUnicode::empty()));

    let mut translator_i = TranslatorI {
        trans: &translator,
        pattern: "pattern",
    };

    let result = translator_i.visit_class_set_item_post(&class_set_item);
    assert!(result.is_ok());
}

