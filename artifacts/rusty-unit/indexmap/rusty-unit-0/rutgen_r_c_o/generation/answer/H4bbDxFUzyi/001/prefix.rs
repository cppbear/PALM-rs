// Answer 0

#[test]
fn test_get_key_value_mut_valid_index() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(0), // assuming index 0 is occupied
        hash_builder: PhantomData,
    };

    occupied_entry.get_key_value_mut();
}

#[test]
#[should_panic]
fn test_get_key_value_mut_empty_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };

    occupied_entry.get_key_value_mut();
}

#[test]
#[should_panic]
fn test_get_key_value_mut_out_of_bounds_index() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "One".to_string());

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(1), // index is out of bounds
        hash_builder: PhantomData,
    };

    occupied_entry.get_key_value_mut();
}

#[test]
fn test_get_key_value_mut_multiple_entries() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());
    map.insert(3, "Three".to_string());

    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::new(2), // valid index
        hash_builder: PhantomData,
    };

    occupied_entry.get_key_value_mut();
}

