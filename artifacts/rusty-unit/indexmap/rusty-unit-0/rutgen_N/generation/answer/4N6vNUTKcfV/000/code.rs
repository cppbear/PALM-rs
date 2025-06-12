// Answer 0

#[test]
fn test_as_slice_empty_map() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        fn new() -> Self {
            Map {
                entries: Vec::new(),
            }
        }

        fn as_entries(&self) -> &[(K, V)] {
            &self.entries
        }

        fn as_slice(&self) -> &[ (K, V) ] {
            self.as_entries()
        }
    }

    let map: Map<i32, i32> = Map::new();
    assert!(map.as_slice().is_empty());
}

#[test]
fn test_as_slice_non_empty_map() {
    struct Map<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> Map<K, V> {
        fn new(entries: Vec<(K, V)>) -> Self {
            Map {
                entries,
            }
        }

        fn as_entries(&self) -> &[(K, V)] {
            &self.entries
        }

        fn as_slice(&self) -> &[(K, V)] {
            self.as_entries()
        }
    }

    let map = Map::new(vec![(1, "a"), (2, "b")]);
    let slice = map.as_slice();
    assert_eq!(slice.len(), 2);
    assert_eq!(slice[0], (1, "a"));
    assert_eq!(slice[1], (2, "b"));
}

