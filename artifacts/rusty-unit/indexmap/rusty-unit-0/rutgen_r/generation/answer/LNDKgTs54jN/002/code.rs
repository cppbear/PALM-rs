// Answer 0

#[test]
fn test_shift_insert_with_panic_on_out_of_bounds_index_for_vacant_entry() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::new();

    // Attempting to insert a new key beyond current len (which is 0)
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(1, 'a', ());
    });

    assert!(result.is_err());
}

#[test]
fn test_shift_insert_with_panic_on_out_of_bounds_index_for_existing_entry() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::new();
    map.insert('a', ());

    // Moving an existing entry at index which is equal to current len (1)
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(1, 'a', ());
    });

    assert!(result.is_err());
} 

#[test]
fn test_shift_insert_with_new_key_at_boundary() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::new();

    // Insert a new key at the end of an empty map
    let result = map.shift_insert(0, 'b', ());
    assert_eq!(result, None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get_index_of(&'b'), Some(0));
}

#[test]
fn test_shift_insert_panic_when_moving_existing_entry_to_invalid_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::new();
    map.insert('x', ());

    // Move existing entry to invalid index (1) when current len is 1
    let result = std::panic::catch_unwind(|| {
        map.shift_insert(1, 'x', ());
    });

    assert!(result.is_err());
} 

#[test]
fn test_shift_insert_for_existing_key_updates_value() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::new();
    map.insert('c', ());
    map.insert('d', ());

    let old_value = map.shift_insert(0, 'c', ());
    assert_eq!(old_value, Some(()));
    assert_eq!(map.get_index_of(&'c'), Some(0));
    assert_eq!(map.get_index_of(&'d'), Some(1));
}

