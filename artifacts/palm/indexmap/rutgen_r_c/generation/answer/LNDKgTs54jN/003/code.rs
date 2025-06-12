// Answer 0

#[test]
fn test_shift_insert_existing_key_move() {
    use indexmap::IndexMap;

    // Initialize an IndexMap with some key-value pairs
    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    // Ensure the initial state is as expected
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get(&'b'), Some(&2));
    assert_eq!(map.get(&'c'), Some(&3));

    // Move the key 'b' to index 0
    let old_value = map.shift_insert(0, 'b', 20);
    assert_eq!(old_value, Some(2)); // old value should be returned
    assert_eq!(map.get(&'b'), Some(&20)); // new value should be updated
    assert_eq!(map.len(), 3); // size should remain the same

    // 'a' should now be in index 1
    assert_eq!(map.get_index_of(&'a'), Some(1));
}

#[test]
#[should_panic]
fn test_shift_insert_invalid_move_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('x', 10);
    map.insert('y', 20);
    
    // Attempt to move 'x' to an invalid index (index >= len)
    map.shift_insert(2, 'x', 100); // This should panic
}

#[test]
fn test_shift_insert_new_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    // Insert new key 'd' at end of the map (index = len)
    let old_value = map.shift_insert(3, 'd', 4);
    assert_eq!(old_value, None); // No old value should be returned
    assert_eq!(map.len(), 4); // The size increases
    assert_eq!(map.get(&'d'), Some(&4)); // Newly inserted value 'd' should be present
}

