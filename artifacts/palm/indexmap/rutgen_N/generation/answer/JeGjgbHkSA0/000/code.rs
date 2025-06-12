// Answer 0

#[test]
fn test_into_keys() {
    struct MyMapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MyMapSlice<K, V> {
        fn into_entries(self: Box<Self>) -> Vec<(K, V)> {
            let entries = self.entries;
            entries
        }
    }

    struct IntoKeys<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> IntoKeys<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            IntoKeys { entries }
        }
    }

    let map_slice = MyMapSlice {
        entries: vec![(1, "a"), (2, "b"), (3, "c")],
    };

    let keys_iterator = Box::new(map_slice).into_keys();

    let keys: Vec<_> = keys_iterator.entries.into_iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

