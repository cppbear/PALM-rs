// Answer 0

#[test]
fn test_shift_remove_finish_valid_index() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> MyMap<K, V> {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {
            for i in start..end {
                if let Some(index) = self.indices.get_mut(i) {
                    *index -= 1;
                }
            }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.decrement_indices(index + 1, self.entries.len());
            let entry = self.entries.remove(index);
            (entry.key, entry.value)
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
            self.indices.push(self.entries.len() - 1);
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert(1, "one");
    my_map.insert(2, "two");
    my_map.insert(3, "three");

    let (key, value) = my_map.shift_remove_finish(1);
    assert_eq!(key, 2);
    assert_eq!(value, "two");
    assert_eq!(my_map.entries.len(), 2);
    assert_eq!(my_map.entries[0].key, 1);
    assert_eq!(my_map.entries[1].key, 3);
}

#[test]
#[should_panic]
fn test_shift_remove_finish_invalid_index() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }

    struct MyMap<K, V> {
        entries: Vec<Entry<K, V>>,
        indices: Vec<usize>,
    }

    impl<K, V> MyMap<K, V> {
        fn new() -> Self {
            MyMap {
                entries: Vec::new(),
                indices: Vec::new(),
            }
        }

        fn decrement_indices(&mut self, start: usize, end: usize) {
            for i in start..end {
                if let Some(index) = self.indices.get_mut(i) {
                    *index -= 1;
                }
            }
        }

        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.decrement_indices(index + 1, self.entries.len());
            let entry = self.entries.remove(index);
            (entry.key, entry.value)
        }

        fn insert(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
            self.indices.push(self.entries.len() - 1);
        }
    }

    let mut my_map = MyMap::new();
    my_map.insert(1, "one");
    my_map.insert(2, "two");

    my_map.shift_remove_finish(2); // This index is out of bounds and should panic.
}

