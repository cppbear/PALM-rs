// Answer 0

#[test]
fn test_pop_with_entries() {
    struct Entry<K, V> {
        key: K,
        value: V,
        hash: usize,
    }

    struct TestMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> TestMap<K, V> {
        pub(crate) fn new() -> Self {
            TestMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        pub(crate) fn push(&mut self, key: K, value: V, hash: usize) {
            self.entries.push(Entry { key, value, hash });
            self.indices.push(self.entries.len() - 1);
        }

        pub(crate) fn pop(&mut self) -> Option<(K, V)> {
            if let Some(entry) = self.entries.pop() {
                let last = self.entries.len();
                erase_index(&mut self.indices, entry.hash, last);
                Some((entry.key, entry.value))
            } else {
                None
            }
        }
    }

    fn erase_index<K>(indices: &mut Vec<usize>, hash: usize, last: usize) {
        // For this example, we will simulate index erasing without actual hash-based logic.
        if last < indices.len() {
            indices.swap_remove(last);
        }
    }

    let mut map = TestMap::new();
    map.push("key1", "value1", 1);
    map.push("key2", "value2", 2);

    let result = map.pop();
    assert_eq!(result, Some(("key2", "value2")));
    assert_eq!(map.entries.len(), 1); // ensure only one entry remains

    let result = map.pop();
    assert_eq!(result, Some(("key1", "value1")));
    assert_eq!(map.entries.len(), 0); // ensure all entries are removed

    let result = map.pop();
    assert_eq!(result, None); // ensure pop on empty returns None
}

