// Answer 0

#[test]
fn test_insert_before_with_occupied_entry_moving_up() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='c').map(|c| (c, ())).collect();

    // Initial state: map = { 'a': (), 'b': (), 'c': () }
    // Moving 'b' to index 1 (which is its current index) will not change its position.
    assert_eq!(map.insert_before(2, 'b', ()), (1, None)); // Move 'b' up to index 1
    assert_eq!(map.get_index_of(&'b'), Some(1)); // 'b' should be at index 1

    // Add a new key at the end which will ensure the index stays within bounds
    assert_eq!(map.insert_before(map.len(), '*', ()), (3, None)); // Add '*' at the end
    assert_eq!(map.get_index_of(&'*'), Some(3)); // '*' should be at index 3

    // Now, moving 'c' before 'b'
    // Since 'c' is at index 2 and we move it to index 1, it should return the old value
    assert_eq!(map.insert_before(2, 'c', ()), (1, Some(()))); // Move 'c' to index 1
    assert_eq!(map.get_index_of(&'c'), Some(1)); // 'c' should now be at index 1
    assert_eq!(map.get_index_of(&'b'), Some(2)); // 'b' should now have moved to index 2
}

