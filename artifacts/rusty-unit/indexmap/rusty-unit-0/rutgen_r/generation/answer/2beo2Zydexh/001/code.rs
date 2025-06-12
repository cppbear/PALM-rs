// Answer 0

#[test]
fn test_values_non_empty_map() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    struct Values<'a, K, V> {
        entries: &'a [(K, V)],
        index: usize,
    }

    impl<'a, K, V> Values<'a, K, V> {
        fn new(entries: &'a [(K, V)]) -> Self {
            Values { entries, index: 0 }
        }
    }

    impl<K, V> MapSlice<K, V> {
        pub fn values(&self) -> Values<'_, K, V> {
            Values::new(&self.entries)
        }
    }

    let map_slice = MapSlice {
        entries: vec![(1, "a"), (2, "b"), (3, "c")],
    };

    let mut values_iter = map_slice.values();
    let values: Vec<_> = values_iter.entries.iter().map(|(_, v)| v).collect();
    assert_eq!(values, vec!["a", "b", "c"]);
}

#[test]
fn test_values_empty_map() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    struct Values<'a, K, V> {
        entries: &'a [(K, V)],
        index: usize,
    }

    impl<'a, K, V> Values<'a, K, V> {
        fn new(entries: &'a [(K, V)]) -> Self {
            Values { entries, index: 0 }
        }
    }

    impl<K, V> MapSlice<K, V> {
        pub fn values(&self) -> Values<'_, K, V> {
            Values::new(&self.entries)
        }
    }

    let map_slice: MapSlice<i32, &str> = MapSlice { entries: vec![] };

    let values_iter = map_slice.values();
    let values: Vec<_> = values_iter.entries.iter().collect();
    assert_eq!(values.len(), 0);
}

