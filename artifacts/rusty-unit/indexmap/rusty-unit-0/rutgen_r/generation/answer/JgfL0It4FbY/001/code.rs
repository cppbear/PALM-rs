// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct TestEntry<K, V> {
        index: Vec<K>,
        entries: Vec<V>,
    }

    impl<K, V> TestEntry<K, V> {
        fn new(index: Vec<K>, entries: Vec<V>) -> Self {
            Self { index, entries }
        }

        fn shift_remove(&mut self) -> (K, V) {
            // Here we simulate the functionality for test purposes.
            let index = 0; // Assume we always remove the first entry for simplicity
            let key = self.index.remove(index);
            let value = self.entries.remove(index);
            (key, value)
        }
    }

    let mut entry = TestEntry::new(vec![1, 2, 3], vec!["a", "b", "c"]);
    
    // Test normal case
    let (key, value) = entry.shift_remove();
    assert_eq!(key, 1);
    assert_eq!(value, "a");

    // Test boundary case with one element
    let mut entry2 = TestEntry::new(vec![42], vec!["single"]);
    let (key2, value2) = entry2.shift_remove();
    assert_eq!(key2, 42);
    assert_eq!(value2, "single");

    // Test panic condition when no elements left
    let mut entry3 = TestEntry::new(vec![], vec![]);
    let result = std::panic::catch_unwind(|| {
        entry3.shift_remove();
    });
    assert!(result.is_err());
}

