// Answer 0

#[test]
fn test_visit_class_set_binary_op_in_unicode() {
    struct MockTranslator {
        flags: Cell<Flags>,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl MockTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
                stack: RefCell::new(vec![]),
            }
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let mut translator = MockTranslator::new();
    let mut visitor = TranslatorI::new(&translator, "");

    let op = ast::ClassSetBinaryOp { /* initialize as necessary */ };

    let result = visitor.visit_class_set_binary_op_in(&op);
    
    assert_eq!(result, Ok(()));
    assert_eq!(translator.stack.borrow().len(), 1);
    if let HirFrame::ClassUnicode(_) = &translator.stack.borrow()[0] {
    } else {
        panic!("Expected ClassUnicode frame");
    }
}

#[test]
fn test_visit_class_set_binary_op_in_bytes() {
    struct MockTranslator {
        flags: Cell<Flags>,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl MockTranslator {
        fn new() -> Self {
            Self {
                flags: Cell::new(Flags { unicode: Some(false), ..Flags::default() }),
                stack: RefCell::new(vec![]),
            }
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let mut translator = MockTranslator::new();
    let mut visitor = TranslatorI::new(&translator, "");

    let op = ast::ClassSetBinaryOp { /* initialize as necessary */ };

    let result = visitor.visit_class_set_binary_op_in(&op);
    
    assert_eq!(result, Ok(()));
    assert_eq!(translator.stack.borrow().len(), 1);
    if let HirFrame::ClassBytes(_) = &translator.stack.borrow()[0] {
    } else {
        panic!("Expected ClassBytes frame");
    }
}

