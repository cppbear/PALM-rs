// Answer 0

#[test]
fn test_finish_non_empty_stack() {
    // Helper struct to create a context required for the test
    struct MockTrans {
        stack: std::cell::RefCell<Vec<Result<Hir>>>,
    }

    impl MockTrans {
        fn new() -> Self {
            MockTrans {
                stack: std::cell::RefCell::new(vec![Ok(Hir::new_expr())]),
            }
        }

        fn borrow_stack(&self) -> std::cell::Ref<'_, Vec<Result<Hir>>> {
            self.stack.borrow()
        }
    }

    struct TestStruct {
        trans: MockTrans,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                trans: MockTrans::new(),
            }
        }

        fn trans(&self) -> &MockTrans {
            &self.trans
        }

        fn pop(&mut self) -> Option<Result<Hir>> {
            self.trans.stack.borrow_mut().pop()
        }
    }

    impl TestStruct {
        fn finish(self) -> Result<Hir> {
            if self.trans.borrow_stack().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans.borrow_stack().len(), 1);
            Ok(self.pop().unwrap().unwrap())
        }
    }

    let test_struct = TestStruct::new();
    let result = test_struct.finish();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_finish_empty_stack() {
    struct MockTrans {
        stack: std::cell::RefCell<Vec<Result<Hir>>>,
    }

    impl MockTrans {
        fn new() -> Self {
            MockTrans {
                stack: std::cell::RefCell::new(vec![]),
            }
        }

        fn borrow_stack(&self) -> std::cell::Ref<'_, Vec<Result<Hir>>> {
            self.stack.borrow()
        }
    }

    struct TestStruct {
        trans: MockTrans,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                trans: MockTrans::new(),
        }
        }

        fn trans(&self) -> &MockTrans {
            &self.trans
        }

        fn pop(&mut self) -> Option<Result<Hir>> {
            self.trans.stack.borrow_mut().pop()
        }
    }

    impl TestStruct {
        fn finish(self) -> Result<Hir> {
            if self.trans.borrow_stack().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans.borrow_stack().len(), 1);
            Ok(self.pop().unwrap().unwrap())
        }
    }

    let test_struct = TestStruct::new();
    test_struct.finish();
}

