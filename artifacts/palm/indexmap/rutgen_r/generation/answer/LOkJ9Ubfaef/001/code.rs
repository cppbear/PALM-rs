// Answer 0

#[test]
fn test_index_valid() {
    struct TestEntry {
        key: String,
    }

    struct TestSlice {
        entries: Vec<TestEntry>,
    }

    impl TestSlice {
        fn index(&self, index: usize) -> &String {
            &self.entries[index].key
        }
    }

    let entries = vec![TestEntry { key: String::from("first") },
                       TestEntry { key: String::from("second") },
                       TestEntry { key: String::from("third") }];
    let slice = TestSlice { entries };

    assert_eq!(slice.index(0), "first");
    assert_eq!(slice.index(1), "second");
    assert_eq!(slice.index(2), "third");
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_lower() {
    struct TestEntry {
        key: String,
    }

    struct TestSlice {
        entries: Vec<TestEntry>,
    }

    impl TestSlice {
        fn index(&self, index: usize) -> &String {
            &self.entries[index].key
        }
    }

    let entries = vec![TestEntry { key: String::from("first") }];
    let slice = TestSlice { entries };

    slice.index(1); // This should trigger a panic
}

#[test]
#[should_panic]
fn test_index_out_of_bounds_upper() {
    struct TestEntry {
        key: String,
    }

    struct TestSlice {
        entries: Vec<TestEntry>,
    }

    impl TestSlice {
        fn index(&self, index: usize) -> &String {
            &self.entries[index].key
        }
    }

    let entries = vec![TestEntry { key: String::from("first") }];
    let slice = TestSlice { entries };

    slice.index(2); // This should trigger a panic
}

