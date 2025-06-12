// Answer 0

#[test]
fn test_shift_insert_hashed_nocheck_valid_case() {
    struct TestEntry {
        key: i32,
        value: String,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn shift_insert_unique(&mut self, index: usize, _hash: usize, key: i32, value: String) {
            self.entries.insert(index, TestEntry { key, value });
        }
    }

    let mut map = TestMap::new();
    let (key_ref, value_ref) = shift_insert_hashed_nocheck(&mut map, 0, 123456, 42, String::from("test"));
    assert_eq!(*key_ref, 42);
    assert_eq!(*value_ref, "test");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_insert_hashed_nocheck_out_of_bounds() {
    struct TestEntry {
        key: i32,
        value: String,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        fn shift_insert_unique(&mut self, index: usize, _hash: usize, key: i32, value: String) {
            self.entries.insert(index, TestEntry { key, value });
        }
    }

    let mut map = TestMap::new();
    shift_insert_hashed_nocheck(&mut map, 1, 123456, 42, String::from("test"));
}

