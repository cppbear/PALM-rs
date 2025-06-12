// Answer 0

#[test]
fn test_iter_empty_map() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, i32> = MapSlice { entries: vec![] };
    let iter = map_slice.iter();

    assert!(!iter.clone().any()); // Ensure iterator is empty
}

#[test]
fn test_iter_single_entry() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, i32> = MapSlice {
        entries: vec![(1, 100)],
    };
    let mut iter = map_slice.iter();

    assert_eq!(iter.next(), Some(&(1, 100))); // Ensure iterator returns the single entry
    assert!(iter.next().is_none()); // Ensure iterator is exhausted
}

#[test]
fn test_iter_multiple_entries() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, i32> = MapSlice {
        entries: vec![(1, 100), (2, 200), (3, 300)],
    };
    let iter = map_slice.iter();

    let results: Vec<_> = iter.collect();
    assert_eq!(results, vec![&(1, 100), &(2, 200), &(3, 300)]); // Ensure all entries are returned
}

