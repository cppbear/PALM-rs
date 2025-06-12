// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestItem {
        value: usize,
    }

    impl TestItem {
        fn new(value: usize) -> Self {
            TestItem { value }
        }
    }

    let entries = [
        Bucket { hash: 0, key: TestItem::new(1), value: () },
        Bucket { hash: 0, key: TestItem::new(2), value: () },
        Bucket { hash: 0, key: TestItem::new(3), value: () },
    ];

    let slice = Slice { entries };

    let result = slice.binary_search_by_key(&2, |item| item.value);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_not_found() {
    struct TestItem {
        value: usize,
    }

    impl TestItem {
        fn new(value: usize) -> Self {
            TestItem { value }
        }
    }

    let entries = [
        Bucket { hash: 0, key: TestItem::new(1), value: () },
        Bucket { hash: 0, key: TestItem::new(2), value: () },
        Bucket { hash: 0, key: TestItem::new(3), value: () },
    ];

    let slice = Slice { entries };

    let result = slice.binary_search_by_key(&4, |item| item.value);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_insert_position() {
    struct TestItem {
        value: usize,
    }

    impl TestItem {
        fn new(value: usize) -> Self {
            TestItem { value }
        }
    }

    let entries = [
        Bucket { hash: 0, key: TestItem::new(1), value: () },
        Bucket { hash: 0, key: TestItem::new(3), value: () },
        Bucket { hash: 0, key: TestItem::new(5), value: () },
    ];

    let slice = Slice { entries };

    let result = slice.binary_search_by_key(&4, |item| item.value);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_by_key_empty_slice() {
    struct TestItem {
        value: usize,
    }

    let entries: [Bucket<TestItem>; 0] = [];
    let slice = Slice { entries };

    let result = slice.binary_search_by_key(&1, |item| item.value);
    assert_eq!(result, Err(0));
}

