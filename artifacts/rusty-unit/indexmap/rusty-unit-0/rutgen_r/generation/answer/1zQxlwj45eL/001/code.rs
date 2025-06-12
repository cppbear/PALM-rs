// Answer 0

#[test]
fn test_shift_insert_valid_index() {
    struct MapEntry<V> {
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<MapEntry<V>>,
    }

    struct ShiftInsert<'a, K, V> {
        map: &'a mut Map<K, V>,
        hash: u64,
        key: K,
    }

    impl<K, V> ShiftInsert<'_, K, V> {
        pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
            self.map.entries.insert(index, MapEntry { value });
            &mut self.map.entries[index].value
        }
    }

    let mut map = Map { entries: vec![] };
    
    // Insert at index 0 (valid index)
    let mut insert = ShiftInsert { map: &mut map, hash: 0, key: 0 };
    let value_ref = insert.shift_insert(0, 5);
    assert_eq!(*value_ref, 5);
    assert_eq!(map.entries.len(), 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_insert_out_of_bounds_index() {
    struct MapEntry<V> {
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<MapEntry<V>>,
    }

    struct ShiftInsert<'a, K, V> {
        map: &'a mut Map<K, V>,
        hash: u64,
        key: K,
    }

    impl<K, V> ShiftInsert<'_, K, V> {
        pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
            self.map.entries.insert(index, MapEntry { value });
            &mut self.map.entries[index].value
        }
    }

    let mut map = Map { entries: vec![] };
    
    // Attempt to insert at index 1 (invalid, since the map is empty)
    let insert = ShiftInsert { map: &mut map, hash: 0, key: 0 };
    let _ = insert.shift_insert(1, 10);
}

#[test]
fn test_shift_insert_multiple_inserts() {
    struct MapEntry<V> {
        value: V,
    }

    struct Map<K, V> {
        entries: Vec<MapEntry<V>>,
    }

    struct ShiftInsert<'a, K, V> {
        map: &'a mut Map<K, V>,
        hash: u64,
        key: K,
    }

    impl<K, V> ShiftInsert<'_, K, V> {
        pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
            self.map.entries.insert(index, MapEntry { value });
            &mut self.map.entries[index].value
        }
    }

    let mut map = Map { entries: vec![] };

    // Inserting multiple entries
    let mut insert = ShiftInsert { map: &mut map, hash: 0, key: 0 };
    let value_ref1 = insert.shift_insert(0, 1); // Insert at index 0
    assert_eq!(*value_ref1, 1);

    let value_ref2 = insert.shift_insert(1, 2); // Insert at index 1
    assert_eq!(*value_ref2, 2);

    let value_ref3 = insert.shift_insert(1, 3); // Insert at index 1, shifting 2 to index 2
    assert_eq!(*value_ref3, 3);
    
    assert_eq!(map.entries.len(), 3);
    assert_eq!(map.entries[1].value, 3);
    assert_eq!(map.entries[2].value, 2);
}

