// Answer 0

#[test]
fn test_pop_empty_stack() {
    struct TestTrans {
        stack: std::cell::RefCell<Vec<HirFrame>>,
    }

    impl TestTrans {
        fn new() -> Self {
            TestTrans {
                stack: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn trans(&self) -> &Self {
            self
        }
    }

    struct HirFrame {}

    let trans = TestTrans::new();
    let result = trans.pop();
    assert_eq!(result, None);
}

#[test]
fn test_pop_single_element_stack() {
    struct TestTrans {
        stack: std::cell::RefCell<Vec<HirFrame>>,
    }

    impl TestTrans {
        fn new(stack: Vec<HirFrame>) -> Self {
            TestTrans {
                stack: std::cell::RefCell::new(stack),
            }
        }

        fn trans(&self) -> &Self {
            self
        }
    }

    struct HirFrame {}

    let frame = HirFrame {};
    let trans = TestTrans::new(vec![frame]);
    let result = trans.pop();
    assert!(result.is_some());
}

#[test]
fn test_pop_multiple_elements_stack() {
    struct TestTrans {
        stack: std::cell::RefCell<Vec<HirFrame>>,
    }

    impl TestTrans {
        fn new(stack: Vec<HirFrame>) -> Self {
            TestTrans {
                stack: std::cell::RefCell::new(stack),
            }
        }

        fn trans(&self) -> &Self {
            self
        }
    }

    struct HirFrame {}

    let frame1 = HirFrame {};
    let frame2 = HirFrame {};
    let trans = TestTrans::new(vec![frame1, frame2]);
    let result = trans.pop();
    assert!(result.is_some());
    let second_result = trans.pop();
    assert!(second_result.is_some());
    let third_result = trans.pop();
    assert_eq!(third_result, None);
}

