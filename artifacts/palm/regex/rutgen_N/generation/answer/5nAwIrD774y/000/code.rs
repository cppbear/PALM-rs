// Answer 0

#[test]
fn test_finish_empty_stack() {
    struct MockTrans {
        stack: std::rc::Rc<std::cell::RefCell<Vec<Option<Hir>>>>,
    }

    impl MockTrans {
        fn new() -> Self {
            MockTrans {
                stack: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
            }
        }

        fn stack(&self) -> std::rc::Rc<std::cell::RefCell<Vec<Option<Hir>>>> {
            self.stack.clone()
        }
    }

    struct Mock {
        trans: MockTrans,
    }

    impl Mock {
        fn new() -> Self {
            Mock {
                trans: MockTrans::new(),
            }
        }

        fn trans(&self) -> &MockTrans {
            &self.trans
        }

        fn pop(&self) -> Option<Option<Hir>> {
            Some(None)
        }

        fn finish(self) -> Result<Hir, &'static str> {
            if self.trans().stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans().stack.borrow().len(), 1);
            Ok(self.pop().unwrap().unwrap_expr())
        }
    }

    let mock = Mock::new();
    let result = mock.finish();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_finish_non_empty_stack() {
    struct MockTrans {
        stack: std::rc::Rc<std::cell::RefCell<Vec<Option<Hir>>>>,
    }

    impl MockTrans {
        fn new() -> Self {
            MockTrans {
                stack: std::rc::Rc::new(std::cell::RefCell::new(vec![Some(Hir::new())])),
            }
        }

        fn stack(&self) -> std::rc::Rc<std::cell::RefCell<Vec<Option<Hir>>>> {
            self.stack.clone()
        }
    }

    struct Mock {
        trans: MockTrans,
    }

    impl Mock {
        fn new() -> Self {
            Mock {
                trans: MockTrans::new(),
            }
        }

        fn trans(&self) -> &MockTrans {
            &self.trans
        }

        fn pop(&self) -> Option<Option<Hir>> {
            None
        }

        fn finish(self) -> Result<Hir, &'static str> {
            if self.trans().stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans().stack.borrow().len(), 1);
            Ok(self.pop().unwrap().unwrap_expr())
        }
    }

    let mock = Mock::new();
    let _ = mock.finish();
}

