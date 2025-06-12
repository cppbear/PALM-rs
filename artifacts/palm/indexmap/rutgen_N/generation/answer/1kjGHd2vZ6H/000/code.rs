// Answer 0

#[test]
fn test_get_key_value() {
    struct MockEntry<'a, K, V> {
        entries: &'a [(K, V)],
        index: usize,
    }

    impl<'a, K: Copy, V: Copy> MockEntry<'a, K, V> {
        fn new(entries: &'a [(K, V)], index: usize) -> Self {
            Self { entries, index }
        }

        fn get_key_value(&self) -> (&K, &V) {
            &self.entries[self.index]
        }
    }

    let entries = vec![(1, "a"), (2, "b"), (3, "c")];
    let mock_entry = MockEntry::new(&entries, 1);
    let (key, value) = mock_entry.get_key_value();

    assert_eq!(*key, 2);
    assert_eq!(*value, "b");
}

#[test]
fn test_get_key_value_boundary() {
    struct MockEntry<'a, K, V> {
        entries: &'a [(K, V)],
        index: usize,
    }

    impl<'a, K: Copy, V: Copy> MockEntry<'a, K, V> {
        fn new(entries: &'a [(K, V)], index: usize) -> Self {
            Self { entries, index }
        }

        fn get_key_value(&self) -> (&K, &V) {
            &self.entries[self.index]
        }
    }

    let entries = vec![(1, "a")];
    let mock_entry = MockEntry::new(&entries, 0);
    let (key, value) = mock_entry.get_key_value();

    assert_eq!(*key, 1);
    assert_eq!(*value, "a");
}

