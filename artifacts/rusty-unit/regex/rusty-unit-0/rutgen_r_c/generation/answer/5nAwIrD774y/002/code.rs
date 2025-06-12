// Answer 0

#[test]
fn test_finish_with_non_empty_stack() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(vec![]),
            }
        }
    }

    struct MockTranslatorI<'t, 'p> {
        trans: &'t MockTranslator,
        pattern: &'p str,
    }

    impl<'t, 'p> MockTranslatorI<'t, 'p> {
        fn new(trans: &'t MockTranslator, pattern: &'p str) -> Self {
            MockTranslatorI { trans, pattern }
        }

        fn finish(self) -> Result<Hir> {
            if self.trans.stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans.stack.borrow().len(), 1);
            Ok(self.trans.stack.borrow_mut().pop().unwrap().unwrap_expr())
        }
    }

    let translator = MockTranslator::new();
    let hir = Hir::empty(); // Initializing an empty Hir, representing the HIR to be pushed.

    translator.stack.borrow_mut().push(HirFrame::Expr(hir.clone())); // Push a valid HIR onto the stack.

    let translator_i = MockTranslatorI::new(&translator, "test_pattern");
    let result = translator_i.finish();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), hir); // Ensure the result matches the expected HIR.
}

#[test]
#[should_panic] // This test checks for the panic condition
fn test_finish_panic_when_stack_is_empty() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(vec![]),
            }
        }
    }

    struct MockTranslatorI<'t, 'p> {
        trans: &'t MockTranslator,
        pattern: &'p str,
    }

    impl<'t, 'p> MockTranslatorI<'t, 'p> {
        fn new(trans: &'t MockTranslator, pattern: &'p str) -> Self {
            MockTranslatorI { trans, pattern }
        }

        fn finish(self) -> Result<Hir> {
            if self.trans.stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans.stack.borrow().len(), 1);
            Ok(self.trans.stack.borrow_mut().pop().unwrap().unwrap_expr())
        }
    }

    let translator = MockTranslator::new();
    let translator_i = MockTranslatorI::new(&translator, "test_pattern");
    translator_i.finish(); // This will panic since stack is empty
}

#[test]
fn test_finish_with_multiple_hir_frames() {
    struct MockTranslator {
        stack: RefCell<Vec<HirFrame>>,
    }

    impl MockTranslator {
        fn new() -> Self {
            MockTranslator {
                stack: RefCell::new(vec![]),
            }
        }
    }

    struct MockTranslatorI<'t, 'p> {
        trans: &'t MockTranslator,
        pattern: &'p str,
    }

    impl<'t, 'p> MockTranslatorI<'t, 'p> {
        fn new(trans: &'t MockTranslator, pattern: &'p str) -> Self {
            MockTranslatorI { trans, pattern }
        }

        fn finish(self) -> Result<Hir> {
            if self.trans.stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans.stack.borrow().len(), 1);
            Ok(self.trans.stack.borrow_mut().pop().unwrap().unwrap_expr())
        }
    }

    let translator = MockTranslator::new();
    let hir1 = Hir::empty();
    let hir2 = Hir::empty(); // Two empty HIRs to test the stack size

    translator.stack.borrow_mut().push(HirFrame::Expr(hir1.clone())); // Only one HIR should be present
    translator.stack.borrow_mut().push(HirFrame::Expr(hir2.clone())); // Try to observe behavior

    let translator_i = MockTranslatorI::new(&translator, "test_pattern");
    let result = translator_i.finish();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), hir2); // Ensure the result matches the expected last HIR.
}

