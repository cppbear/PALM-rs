// Answer 0

#[test]
fn test_finish_non_empty_stack() {
    struct Trans {
        stack: std::cell::RefCell<Vec<Option<Hir>>>,
    }

    impl Trans {
        fn new() -> Self {
            Trans {
                stack: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn push(&self, value: Option<Hir>) {
            self.stack.borrow_mut().push(value);
        }

        fn pop(&self) -> Option<Option<Hir>> {
            self.stack.borrow_mut().pop()
        }
    }

    struct Hir;

    impl Hir {
        fn empty() -> Result<Hir, ()> {
            Ok(Hir)
        }

        fn unwrap_expr(self) -> Hir {
            self
        }
    }

    struct TestStruct {
        trans: Trans,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                trans: Trans::new(),
            }
        }

        fn trans(&self) -> &Trans {
            &self.trans
        }

        fn pop(&self) -> Option<Option<Hir>> {
            self.trans.pop()
        }

        fn finish(self) -> Result<Hir, ()> {
            if self.trans().stack.borrow().is_empty() {
                return Ok(Hir::empty()?);
            }
            assert_eq!(self.trans().stack.borrow().len(), 1);
            Ok(self.pop().unwrap().unwrap_expr())
        }
    }

    let test_struct = TestStruct::new();
    let hir_value = Some(Hir);
    test_struct.trans.push(hir_value.clone());

    assert_eq!(test_struct.finish().is_ok(), true);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_finish_empty_stack_panics() {
    struct Trans {
        stack: std::cell::RefCell<Vec<Option<Hir>>>,
    }

    impl Trans {
        fn new() -> Self {
            Trans {
                stack: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn pop(&self) -> Option<Option<Hir>> {
            self.stack.borrow_mut().pop()
        }
    }

    struct Hir;

    struct TestStruct {
        trans: Trans,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                trans: Trans::new(),
            }
        }

        fn trans(&self) -> &Trans {
            &self.trans
        }

        fn pop(&self) -> Option<Option<Hir>> {
            self.trans.pop()
        }

        fn finish(self) -> Result<Hir, ()> {
            if self.trans().stack.borrow().is_empty() {
                return Ok(Hir::empty().unwrap());
            }
            assert_eq!(self.trans().stack.borrow().len(), 1);
            Ok(self.pop().unwrap().unwrap_expr())
        }
    }

    let test_struct = TestStruct::new();
    assert_eq!(test_struct.finish().is_ok(), true);
}

