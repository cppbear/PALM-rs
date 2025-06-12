// Answer 0

#[test]
fn test_insert_sorted_with_empty_map() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K: Ord, V> TestMap<K, V> {
        fn insert_sorted(&mut self, key: K, value: V) -> (usize, &mut V) {
            let i = self.entries.binary_search_by(|(k, _)| k.cmp(&key)).unwrap_err();
            self.entries.insert(i, (key, value));
            let entry = &mut self.entries[i].1; 
            (i, entry)
        }
    }

    let mut map = TestMap { entries: Vec::new() };
    let (index, value_ref) = map.insert_sorted(1, "value1");

    assert_eq!(index, 0);
    assert_eq!(*value_ref, "value1");
}

#[test]
fn test_insert_sorted_with_sorted_map() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K: Ord, V> TestMap<K, V> {
        fn insert_sorted(&mut self, key: K, value: V) -> (usize, &mut V) {
            let i = self.entries.binary_search_by(|(k, _)| k.cmp(&key)).unwrap_err();
            self.entries.insert(i, (key, value));
            let entry = &mut self.entries[i].1; 
            (i, entry)
        }
    }

    let mut map = TestMap {
        entries: vec![(1, "value1"), (3, "value3")],
    };
    let (index, value_ref) = map.insert_sorted(2, "value2");

    assert_eq!(index, 1);
    assert_eq!(*value_ref, "value2");
    assert_eq!(map.entries, vec![(1, "value1"), (2, "value2"), (3, "value3")]);
}

#[test]
fn test_insert_sorted_with_duplicate_key() {
    struct TestMap<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K: Ord, V> TestMap<K, V> {
        fn insert_sorted(&mut self, key: K, value: V) -> (usize, &mut V) {
            let i = self.entries.binary_search_by(|(k, _)| k.cmp(&key)).unwrap_err();
            self.entries.insert(i, (key, value));
            let entry = &mut self.entries[i].1; 
            (i, entry)
        }
    }

    let mut map = TestMap {
        entries: vec![(1, "value1")],
    };
    let (index, value_ref) = map.insert_sorted(1, "value2");

    assert_eq!(index, 1);
    assert_eq!(*value_ref, "value2");
    assert_eq!(map.entries, vec![(1, "value1"), (1, "value2")]);
}

