// Answer 0

#[test]
fn test_get_mut_valid_index() {
    struct MapEntry<V> {
        value: V,
    }

    struct IndexedMap<V> {
        entries: Vec<MapEntry<V>>,
    }

    struct IndexedEntry<'a, V> {
        map: &'a mut IndexedMap<V>,
        index: usize,
    }

    impl<V> IndexedEntry<'_, V> {
        pub fn get_mut(&mut self) -> &mut V {
            &mut self.map.entries[self.index].value
        }
    }

    // Create a map with a valid index and value
    let mut indexed_map = IndexedMap {
        entries: vec![MapEntry { value: 1 }, MapEntry { value: 2 }],
    };
    
    let mut indexed_entry = IndexedEntry { map: &mut indexed_map, index: 0 };
    let value_mut_ref = indexed_entry.get_mut();
    *value_mut_ref += 1; // Modifying the value
    assert_eq!(indexed_map.entries[0].value, 2);
}

#[test]
#[should_panic]
fn test_get_mut_out_of_bounds_index() {
    struct MapEntry<V> {
        value: V,
    }

    struct IndexedMap<V> {
        entries: Vec<MapEntry<V>>,
    }

    struct IndexedEntry<'a, V> {
        map: &'a mut IndexedMap<V>,
        index: usize,
    }

    impl<V> IndexedEntry<'_, V> {
        pub fn get_mut(&mut self) -> &mut V {
            &mut self.map.entries[self.index].value
        }
    }

    // Create a map with no entries
    let mut indexed_map = IndexedMap { entries: vec![] };
    
    // Attempt to create an indexed entry with an invalid index
    let mut indexed_entry = IndexedEntry { map: &mut indexed_map, index: 0 };
    indexed_entry.get_mut(); // This should panic
}

