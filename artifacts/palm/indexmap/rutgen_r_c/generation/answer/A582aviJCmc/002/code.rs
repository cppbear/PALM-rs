// Answer 0

#[test]
fn test_insert_before_with_occupied_entry_and_shift() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', ());
    map.insert('b', ());
    map.insert('c', ());

    // Initial length of the map is 3 (containing 'a', 'b', 'c')
    // Inserting before index 2 where 'b' is located
    let (index, old_value) = map.insert_before(2, 'b', ());
    
    assert_eq!(old_value, Some(())); // Should return the old value of 'b'
    assert_eq!(index, 1); // 'b' should move to index 1
    assert_eq!(map.len(), 3); // The length of the map remains the same
    assert_eq!(map.get_index_of(&'b'), Some(1)); // 'b' is now at index 1
    assert_eq!(map.get_index_of(&'a'), Some(0)); // 'a' is still at index 0
    assert_eq!(map.get_index_of(&'c'), Some(2)); // 'c' moves to index 2
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 4. Expected index <= len")]
fn test_insert_before_out_of_bounds() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', ());
    map.insert('b', ());
    map.insert('c', ());

    // This should panic because the index 4 exceeds the current length of 3
    map.insert_before(4, 'd', ());
}

#[test]
fn test_insert_before_with_occupied_entry_no_shift() {
    use std::collections::hash_map::RandomState;
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', ());
    map.insert('b', ());
    map.insert('c', ());
    
    // Inserting before index 1 where 'b' is located
    let (index, old_value) = map.insert_before(1, 'a', ());
    
    assert_eq!(old_value, Some(())); // Should return the old value of 'a'
    assert_eq!(index, 0); // 'a' is at index 0, it moves to the same index
    assert_eq!(map.len(), 3); // The length of the map remains the same
    assert_eq!(map.get_index_of(&'a'), Some(0)); // 'a' remains at index 0
    assert_eq!(map.get_index_of(&'b'), Some(1)); // 'b' moves down to index 1
    assert_eq!(map.get_index_of(&'c'), Some(2)); // 'c' is still at index 2
}

