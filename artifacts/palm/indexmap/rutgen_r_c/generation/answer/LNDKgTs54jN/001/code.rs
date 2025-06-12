// Answer 0

#[test]
fn test_shift_insert_vacant_at_end() {
    use indexmap::IndexMap;

    // Create an IndexMap with some initial values.
    let mut map: IndexMap<char, usize> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);
    
    // The current length of the map.
    let len = map.len();
    
    // Insert at the end of the map (index equals len).
    let result = map.shift_insert(len, 'd', 4);
    
    // Assert that the function returns None as expected.
    assert_eq!(result, None);
    // Check that the new key-value pair is inserted.
    assert_eq!(map.get(&'d'), Some(&4));
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn test_shift_insert_existing_key_invalid_move() {
    use indexmap::IndexMap;

    // Create an IndexMap with some initial values.
    let mut map: IndexMap<char, usize> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    map.insert('c', 3);

    // Attempt to move an existing key to an invalid index (index == len).
    let _ = map.shift_insert(3, 'a', 5);
}

