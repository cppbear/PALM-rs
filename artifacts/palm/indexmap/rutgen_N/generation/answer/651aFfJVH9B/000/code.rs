// Answer 0

#[test]
fn test_index_valid() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new(entries: Vec<TestEntry>) -> Self {
            TestMap { entries }
        }

        fn index(&self, index: usize) -> &i32 {
            &self.entries[index].value
        }
    }

    let entries = vec![TestEntry { value: 1 }, TestEntry { value: 2 }, TestEntry { value: 3 }];
    let test_map = TestMap::new(entries);

    assert_eq!(*test_map.index(0), 1);
    assert_eq!(*test_map.index(1), 2);
    assert_eq!(*test_map.index(2), 3);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds() {
    struct TestEntry {
        value: i32,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new(entries: Vec<TestEntry>) -> Self {
            TestMap { entries }
        }

        fn index(&self, index: usize) -> &i32 {
            &self.entries[index].value
        }
    }

    let entries = vec![TestEntry { value: 1 }];
    let test_map = TestMap::new(entries);

    // This will panic since the index is out of bounds
    let _ = test_map.index(1);
}

