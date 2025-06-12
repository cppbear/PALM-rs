// Answer 0

#[test]
fn test_swap_remove_entry() {
    struct TestEntry {
        index: usize,
        entries: Vec<(i32, String)>,
    }

    impl TestEntry {
        fn new(index: usize, entries: Vec<(i32, String)>) -> Self {
            TestEntry { index, entries }
        }

        fn remove(&mut self) -> (usize, (i32, String)) {
            let index = self.index;
            let entry = self.entries.remove(index);
            (index, entry)
        }
    }

    struct TestMap {
        entries: TestEntry,
    }

    impl TestMap {
        fn new(entries: Vec<(i32, String)>) -> Self {
            TestMap { entries: TestEntry::new(0, entries) }  // Start with index 0 for testing
        }

        fn swap_remove_entry(self) -> (i32, String) {
            let (index, entry) = self.entries.remove();
            self.entries.entries.swap_remove(index);
            entry
        }
    }

    let mut test_map = TestMap::new(vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())]);
    let result = test_map.swap_remove_entry();
    assert_eq!(result, (1, "one".to_string()));
    assert_eq!(test_map.entries.entries.len(), 2);
}

