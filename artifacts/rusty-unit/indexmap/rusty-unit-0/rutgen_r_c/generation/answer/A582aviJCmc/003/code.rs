// Answer 0

#[test]
fn test_insert_before_occupied_entry_at_same_index() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    
    // Insert an occupied key at the same index as its current position
    let (index, old_value) = map.insert_before(1, 'b', 20);
    
    assert_eq!(index, 1);
    assert_eq!(old_value, Some(2)); // Old value for key 'b'
    assert_eq!(map.get(&'b'), Some(&20)); // New value for key 'b'
}

#[test]
fn test_insert_before_occupied_entry_index_adjustment() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    
    // Move the occupied key down in the map
    let (index, old_value) = map.insert_before(2, 'a', 10);
    
    assert_eq!(index, 1); // 'a' moves to index 1
    assert_eq!(old_value, Some(1)); // Old value for key 'a'
    assert_eq!(map.get(&'a'), Some(&10)); // New value for key 'a'
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 4. Expected index <= len")]
fn test_insert_before_out_of_bounds() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    
    // Attempt to insert with index out of bounds
    let _ = map.insert_before(4, 'd', 4);
}

#[test]
fn test_insert_before_same_index_with_multiple_occupied_keys() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    
    // Insert key 'b' at its current position, which should not change its position
    let (index, old_value) = map.insert_before(1, 'b', 20);
    
    assert_eq!(index, 1);
    assert_eq!(old_value, Some(2)); // Old value for key 'b'
    assert_eq!(map.get(&'b'), Some(&20)); // New value for key 'b'
    
    // Inserting 'c' before 'b' should adjust its position
    let (index2, old_value2) = map.insert_before(1, 'c', 30);
    
    assert_eq!(index2, 1); // 'c' moves to index 1
    assert_eq!(old_value2, Some(3)); // Old value for key 'c'
    assert_eq!(map.get(&'c'), Some(&30)); // New value for key 'c'
}

