// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_unicode() {
    use ast::{ClassSetItem, ClassAsciiKind, Literal, Flags, ClassUnicode, ClassUnicodeRange};

    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            Self {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags { unicode: Some(true), ..Default::default() })
            }
        }
    }

    let translator = MockTranslator::new();
    let mut visitor = TranslatorI::new(&translator, "");

    let ast_item = ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ClassAsciiKind::Alnum,
        negated: false,
    });

    // Push an empty ClassUnicode frame to the translator's stack
    translator.stack.borrow_mut().push(HirFrame::ClassUnicode(ClassUnicode::empty()));

    // Call the function under test
    let result = visitor.visit_class_set_item_post(&ast_item);
    
    // Check expected result
    assert!(result.is_ok());

    // Verify the class in the translator's stack after processing
    let frame = translator.stack.borrow_mut().pop().unwrap().unwrap_class_unicode();
    let expected_ranges = ascii_class(&ClassAsciiKind::Alnum);
    assert_eq!(frame.ranges().len(), expected_ranges.len());

    for ((s, e), range) in expected_ranges.iter().zip(frame.ranges().iter()) {
        assert_eq!(range.start(), *s);
        assert_eq!(range.end(), *e);
    }
}

#[test]
fn test_visit_class_set_item_post_ascii_unicode_negated() {
    use ast::{ClassSetItem, ClassAsciiKind, Literal, Flags, ClassUnicode, ClassUnicodeRange};

    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
    }

    impl MockTranslator {
        fn new() -> Self {
            Self {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags { unicode: Some(true), ..Default::default() })
            }
        }
    }

    let translator = MockTranslator::new();
    let mut visitor = TranslatorI::new(&translator, "");

    let ast_item = ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ClassAsciiKind::Digit,
        negated: true,
    });

    translator.stack.borrow_mut().push(HirFrame::ClassUnicode(ClassUnicode::empty()));

    let result = visitor.visit_class_set_item_post(&ast_item);

    assert!(result.is_ok());

    let frame = translator.stack.borrow_mut().pop().unwrap().unwrap_class_unicode();
    let expected_ranges = ascii_class(&ClassAsciiKind::Digit);
    assert_eq!(frame.ranges().len(), expected_ranges.len());

    for ((s, e), range) in expected_ranges.iter().zip(frame.ranges().iter()) {
        assert_eq!(range.start(), *s);
        assert_eq!(range.end(), *e);
    }
}

#[test]
fn test_visit_class_set_item_post_ascii_bytes() {
    use ast::{ClassSetItem, ClassAsciiKind, Literal, Flags, ClassBytes, ClassBytesRange};
    
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
        allow_invalid_utf8: bool,
    }

    impl MockTranslator {
        fn new() -> Self {
            Self {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags { unicode: Some(false), ..Default::default() }),
                allow_invalid_utf8: false,
            }
        }
    }

    let translator = MockTranslator::new();
    let mut visitor = TranslatorI::new(&translator, "");

    let ast_item = ClassSetItem::Ascii(ast::ClassAscii {
        span: Span { start: Position(0), end: Position(1) },
        kind: ClassAsciiKind::Digit,
        negated: false,
    });

    translator.stack.borrow_mut().push(HirFrame::ClassBytes(ClassBytes::empty()));

    let result = visitor.visit_class_set_item_post(&ast_item);

    assert!(result.is_ok());

    let frame = translator.stack.borrow_mut().pop().unwrap().unwrap_class_bytes();
    let expected_ranges = ascii_class(&ClassAsciiKind::Digit);
    assert_eq!(frame.ranges().len(), expected_ranges.len());

    for ((s, e), range) in expected_ranges.iter().zip(frame.ranges().iter()) {
        assert_eq!(range.start(), *s as u8);
        assert_eq!(range.end(), *e as u8);
    }
}

