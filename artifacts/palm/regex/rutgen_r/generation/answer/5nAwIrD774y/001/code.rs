// Answer 0

#[test]
fn test_finish_empty_stack() {
    struct MockTrans {
        stack: std::cell::RefCell<Vec<Option<Hir>>>,
    }

    impl MockTrans {
        fn new() -> Self {
            MockTrans {
                stack: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn stack(&self) -> &std::cell::RefCell<Vec<Option<Hir>>> {
            &self.stack
        }
    }

    struct MockSelf {
        trans: MockTrans,
    }

    impl MockSelf {
        fn new(trans: MockTrans) -> Self {
            MockSelf { trans }
        }

        fn trans(&self) -> &MockTrans {
            &self.trans
        }

        fn finish(self) -> Result<Hir, ()> {
            if self.trans().stack.borrow().is_empty() {
                return Ok(Hir::empty());
            }
            assert_eq!(self.trans().stack.borrow().len(), 1);
            Ok(self.pop().unwrap().unwrap_expr())
        }

        fn pop(&self) -> Option<Option<Hir>> {
            self.trans.stack.borrow_mut().pop()
        }
    }

    let mock_trans = MockTrans::new();
    let mock_self = MockSelf::new(mock_trans);
    let result = mock_self.finish();
    
    assert_eq!(result, Ok(Hir::empty()));
}

