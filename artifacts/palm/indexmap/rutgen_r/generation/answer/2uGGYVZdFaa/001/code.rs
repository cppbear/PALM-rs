// Answer 0

#[test]
fn test_shift_remove_entry_non_empty() {
    struct Entry<K, V> {
        index: Vec<(K, V)>,
    }

    impl<K, V> Entry<K, V> {
        fn remove(&mut self) -> (usize, (K, V)) {
            let index = 0;
            let value = self.index.remove(index);
            (index, value)
        }
    }

    struct Table<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Table<K, V> {
        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.entries.remove(index)
        }

        fn new(entries: Vec<(K, V)>) -> Self {
            Self { entries }
        }
    }

    struct TestMap<K, V> {
        table: Table<K, V>,
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            let table = Table::new(entries.clone());
            Self { table, entries }
        }

        fn shift_remove_entry(self) -> (K, V) {
            let (index, entry) = self.table.entries.remove(0);
            self.table.shift_remove_finish(index)
        }
    }

    let mut map = TestMap::new(vec![(1, "a"), (2, "b"), (3, "c")]);
    let result = map.shift_remove_entry();
    assert_eq!(result, (1, "a")); // Assert the first element was removed
}

#[test]
fn test_shift_remove_entry_empty() {
    struct Entry<K, V> {
        index: Vec<(K, V)>,
    }

    impl<K, V> Entry<K, V> {
        fn remove(&mut self) -> (usize, (K, V)) {
            let index = 0; // Following similar implementation
            let value = self.index.remove(index);
            (index, value)
        }
    }

    struct Table<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Table<K, V> {
        fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
            self.entries.remove(index)
        }

        fn new(entries: Vec<(K, V)>) -> Self {
            Self { entries }
        }
    }

    struct TestMap<K, V> {
        table: Table<K, V>,
        entries: Vec<(K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            let table = Table::new(entries.clone());
            Self { table, entries }
        }

        fn shift_remove_entry(self) -> (K, V) {
            let (index, entry) = self.table.entries.remove(0);
            self.table.shift_remove_finish(index)
        }
    }

    let map = TestMap::new(vec![]);
    let result = std::panic::catch_unwind(|| map.shift_remove_entry());
    assert!(result.is_err()); // Assert that it panics due to empty map
}

