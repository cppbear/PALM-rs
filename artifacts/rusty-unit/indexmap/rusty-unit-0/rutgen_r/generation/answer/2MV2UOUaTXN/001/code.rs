// Answer 0

#[test]
fn test_index_within_bounds() {
    struct TestKey {
        value: i32,
    }

    struct TestValue {
        value: String,
    }

    struct TestIter {
        iter: Vec<(TestKey, TestValue)>,
    }

    impl TestIter {
        fn as_slice(&self) -> &[(TestKey, TestValue)] {
            &self.iter
        }
    }

    let test_iter = TestIter {
        iter: vec![
            (TestKey { value: 1 }, TestValue { value: "one".to_string() }),
            (TestKey { value: 2 }, TestValue { value: "two".to_string() }),
            (TestKey { value: 3 }, TestValue { value: "three".to_string() }),
        ],
    };

    assert_eq!(test_iter.as_slice()[0].0.value, 1);
    assert_eq!(test_iter.as_slice()[1].0.value, 2);
    assert_eq!(test_iter.as_slice()[2].0.value, 3);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_lower() {
    struct TestKey {
        value: i32,
    }

    struct TestValue {
        value: String,
    }

    struct TestIter {
        iter: Vec<(TestKey, TestValue)>,
    }

    impl TestIter {
        fn as_slice(&self) -> &[(TestKey, TestValue)] {
            &self.iter
        }
    }

    let test_iter = TestIter {
        iter: vec![
            (TestKey { value: 1 }, TestValue { value: "one".to_string() }),
        ],
    };

    let _panic_value = test_iter.as_slice()[usize::max_value()];
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_upper() {
    struct TestKey {
        value: i32,
    }

    struct TestValue {
        value: String,
    }

    struct TestIter {
        iter: Vec<(TestKey, TestValue)>,
    }

    impl TestIter {
        fn as_slice(&self) -> &[(TestKey, TestValue)] {
            &self.iter
        }
    }

    let test_iter = TestIter {
        iter: vec![
            (TestKey { value: 1 }, TestValue { value: "one".to_string() }),
        ],
    };

    let _panic_value = test_iter.as_slice()[1]; // this index is out of bounds
}

