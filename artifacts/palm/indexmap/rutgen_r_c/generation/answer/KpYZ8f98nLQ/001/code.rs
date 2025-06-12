// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestKey;
    struct TestValue;

    impl Ord for TestKey {
        fn cmp(&self, _: &Self) -> Ordering {
            Ordering::Equal
        }
    }

    impl Ord for TestValue {
        fn cmp(&self, _: &Self) -> Ordering {
            Ordering::Equal
        }
    }

    let entries: Vec<Bucket<TestKey, TestValue>> = vec![Bucket { hash: 0, key: TestKey, value: TestValue }];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by_key(&TestKey, |_, _| TestKey);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_key_insert_position() {
    struct TestKey(usize);
    struct TestValue;

    impl Ord for TestKey {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    let entries: Vec<Bucket<TestKey, TestValue>> = vec![
        Bucket { hash: 0, key: TestKey(1), value: TestValue },
        Bucket { hash: 1, key: TestKey(3), value: TestValue },
        Bucket { hash: 2, key: TestKey(5), value: TestValue },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by_key(&TestKey(4), |_, _| TestKey(4));
    assert_eq!(result, Err(2)); // Should return the position where it can be inserted
}

#[test]
fn test_binary_search_by_key_empty_slice() {
    struct TestKey;
    struct TestValue;

    let entries: Vec<Bucket<TestKey, TestValue>> = vec![];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by_key(&TestKey, |_, _| TestKey);
    assert_eq!(result, Err(0)); // On empty, should return Err(0) for insert position
}

#[test]
#[should_panic]
fn test_binary_search_by_key_out_of_bounds() {
    struct TestKey(usize);
    struct TestValue;

    impl Ord for TestKey {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }

    let entries: Vec<Bucket<TestKey, TestValue>> = vec![
        Bucket { hash: 0, key: TestKey(1), value: TestValue },
        Bucket { hash: 1, key: TestKey(3), value: TestValue },
        Bucket { hash: 2, key: TestKey(5), value: TestValue },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };

    // This will panic because we're trying to access a position out of bounds
    let _ = slice.binary_search_by_key(&TestKey(6), |_, _| TestKey(6));
}

