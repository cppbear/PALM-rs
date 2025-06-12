// Answer 0

#[test]
fn test_move_index_within_bounds() {
    struct TestEntry {
        index: usize,
        map: IndexMap<usize, String>,
    }

    impl TestEntry {
        fn new(index: usize, map: IndexMap<usize, String>) -> Self {
            TestEntry { index, map }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(self, to: usize) {
            self.map.move_index(self.index, to);
        }
    }

    let mut map = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let entry = TestEntry::new(1, map);
    entry.move_index(0); // Move from index 1 to index 0
    assert_eq!(entry.map.get(&0), Some(&"one".to_string())); // Check entry has moved to the new index
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_high() {
    struct TestEntry {
        index: usize,
        map: IndexMap<usize, String>,
    }

    impl TestEntry {
        fn new(index: usize, map: IndexMap<usize, String>) -> Self {
            TestEntry { index, map }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(self, to: usize) {
            self.map.move_index(self.index, to);
        }
    }

    let mut map = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    let entry = TestEntry::new(1, map);
    entry.move_index(3); // Move to index 3, which is out of bounds
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_move_index_out_of_bounds_low() {
    struct TestEntry {
        index: usize,
        map: IndexMap<usize, String>,
    }

    impl TestEntry {
        fn new(index: usize, map: IndexMap<usize, String>) -> Self {
            TestEntry { index, map }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn move_index(self, to: usize) {
            self.map.move_index(self.index, to);
        }
    }

    let mut map = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());

    let entry = TestEntry::new(0, map);
    entry.move_index(!0); // Move to index -1 (invalid)
}

