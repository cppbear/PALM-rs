// Answer 0

fn test_visit_class_set_binary_op_post_unicode_intersection() -> Result<()> {
    struct DummyClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    impl DummyClassSetBinaryOp {
        fn new(kind: ast::ClassSetBinaryOpKind) -> Self {
            DummyClassSetBinaryOp { kind }
        }
    }

    struct DummyTranslator {
        stack: RefCell<Vec<HirFrame>>,
        flags: Cell<Flags>,
    }

    impl DummyTranslator {
        fn new() -> Self {
            DummyTranslator {
                stack: RefCell::new(vec![]),
                flags: Cell::new(Flags {
                    unicode: Some(true),
                    case_insensitive: Some(false),
                    ..Flags::default()
                }),
            }
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }

        fn pop(&self) -> Option<HirFrame> {
            self.stack.borrow_mut().pop()
        }

        fn flags(&self) -> Flags {
            self.flags.get()
        }
    }

    let translator = DummyTranslator::new();

    let op = DummyClassSetBinaryOp::new(ast::ClassSetBinaryOpKind::Intersection);
    let mut class_unicode1 = ClassUnicode::new(vec![]); // Empty class for lhs
    let mut class_unicode2 = ClassUnicode::new(vec![]); // Empty class for rhs
    let mut class_unicode3 = ClassUnicode::new(vec![]); // Empty class for cls

    translator.push(HirFrame::ClassUnicode(class_unicode3));
    translator.push(HirFrame::ClassUnicode(class_unicode1));
    translator.push(HirFrame::ClassUnicode(class_unicode2));

    let visit_result = translator.visit_class_set_binary_op_post(&op);
    assert_eq!(visit_result, Ok(()));
    Ok(())
}

