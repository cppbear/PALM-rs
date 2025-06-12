// Answer 0

#[test]
fn test_iter_empty_map_slice() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, &str> = MapSlice { entries: Vec::new() };
    let iter = map_slice.iter();
    assert!(!iter.clone().any()); // Should be empty
}

#[test]
fn test_iter_single_entry_map_slice() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, &str> = MapSlice { entries: vec![(1, "one")] };
    let mut iter = map_slice.iter();
    assert_eq!(iter.next(), Some(&(1, "one")));
    assert!(iter.next().is_none()); // No more entries
}

#[test]
fn test_iter_multiple_entries_map_slice() {
    struct MapSlice<K, V> {
        entries: Vec<(K, V)>,
    }

    impl<K, V> MapSlice<K, V> {
        pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
            self.entries.iter()
        }
    }

    let map_slice: MapSlice<i32, &str> = MapSlice { entries: vec![(1, "one"), (2, "two"), (3, "three")] };
    let mut iter = map_slice.iter();

    assert_eq!(iter.next(), Some(&(1, "one")));
    assert_eq!(iter.next(), Some(&(2, "two")));
    assert_eq!(iter.next(), Some(&(3, "three")));
    assert!(iter.next().is_none()); // No more entries
}

