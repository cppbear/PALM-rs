// Answer 0

#[test]
fn test_shift_remove_finish_valid_index() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IndexMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> IndexMap<K, V> {
        fn new() -> Self {
            IndexMap { entries: Vec::new() }
        }

        fn push(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {
            for i in start..end {
                // Just a placeholder, no actual indexing logic needed for this test.
            }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.decrement_indices(index + 1, self.entries.len());
            let entry = self.entries.remove(index);
            (entry.key, entry.value)
        }
    }

    let mut map = IndexMap::new();
    map.push(1, "one");
    map.push(2, "two");
    map.push(3, "three");

    let result = map.shift_remove_finish(1);
    assert_eq!(result, (2, "two"));
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].key, 1);
    assert_eq!(map.entries[0].value, "one");
    assert_eq!(map.entries[1].key, 3);
    assert_eq!(map.entries[1].value, "three");
}

#[test]
#[should_panic]
fn test_shift_remove_finish_out_of_bounds() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IndexMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> IndexMap<K, V> {
        fn new() -> Self {
            IndexMap { entries: Vec::new() }
        }
        
        fn push(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {}

        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.decrement_indices(index + 1, self.entries.len());
            let entry = self.entries.remove(index);
            (entry.key, entry.value)
        }
    }

    let mut map = IndexMap::new();
    map.push(1, "one");

    // This should panic due to being out of bounds.
    let _ = map.shift_remove_finish(2);
}

#[test]
fn test_shift_remove_finish_empty() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct IndexMap<K, V> {
        entries: Vec<Entry<K, V>>,
    }

    impl<K, V> IndexMap<K, V> {
        fn new() -> Self {
            IndexMap { entries: Vec::new() }
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {}

        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.decrement_indices(index + 1, self.entries.len());
            let entry = self.entries.remove(index);
            (entry.key, entry.value)
        }
    }

    let mut map: IndexMap<i32, &str> = IndexMap::new();

    // Can't remove from an empty map, should panic.
    let _ = map.shift_remove_finish(0);
}

