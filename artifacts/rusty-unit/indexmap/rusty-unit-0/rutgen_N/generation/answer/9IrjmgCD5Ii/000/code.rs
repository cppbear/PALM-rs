// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K: Clone, V> TestMap<K, V> {
        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.len() {
                let entry = self.entries.remove(index);
                Some(entry)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap<K, V>,
        index: usize,
    }

    impl<K: Clone, V> Entry<K, V> {
        fn shift_remove_entry(mut self) -> (K, V) {
            self.map.shift_remove_index(self.index).unwrap()
        }
    }

    let mut map = TestMap {
        entries: vec![(1, "a"), (2, "b"), (3, "c")],
    };
    let entry = Entry { map: map, index: 1 }; // Remove the entry with key 2, value "b"
    
    let (key, value) = entry.shift_remove_entry();
    
    assert_eq!(key, 2);
    assert_eq!(value, "b");
    assert_eq!(entry.map.entries, vec![(1, "a"), (3, "c")]);
}

#[test]
fn test_shift_remove_entry_out_of_bounds() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K: Clone, V> TestMap<K, V> {
        fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
            if index < self.entries.len() {
                let entry = self.entries.remove(index);
                Some(entry)
            } else {
                None
            }
        }
    }

    struct Entry<K, V> {
        map: TestMap<K, V>,
        index: usize,
    }

    impl<K: Clone, V> Entry<K, V> {
        fn shift_remove_entry(mut self) -> (K, V) {
            self.map.shift_remove_index(self.index).unwrap()
        }
    }

    let mut map = TestMap {
        entries: vec![(1, "a"), (2, "b"), (3, "c")],
    };
    
    let entry = Entry { map: map, index: 5 }; // Out of bounds index
    
    assert!(entry.shift_remove_entry().is_empty());
}

