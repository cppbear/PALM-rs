// Answer 0

#[test]
fn test_insert_before_with_occupied_entry_and_same_index() {
    use indexmap::IndexMap;

    // Initialize the map with some key-value pairs
    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    
    // Existing key 'a' is occupied, we'll try to insert before 'b' (index 1)
    let index = 1; // The index where we are inserting
    let key = 'a'; // Key to be moved
    let value = 10; // New value to assign

    // Ensure 'a' is at index 0 before we call insert_before
    assert_eq!(map.get_index_of(&'a'), Some(0));
    
    // Insert 'a' before index 1 with a new value, expecting it to stay at index 0
    let (new_index, old_value) = map.insert_before(index, key, value);

    // Check the returned index and old value
    assert_eq!(new_index, 0);
    assert_eq!(old_value, Some(1)); // Old value of 'a' which was 1

    // Check the position of 'a' after insert
    assert_eq!(map.get_index_of(&'a'), Some(0));
    assert_eq!(map.get_index_of(&'b'), Some(1)); // 'b' is now at index 1
}

#[test]
fn test_insert_before_with_occupied_entry_at_length() {
    use indexmap::IndexMap;

    // Initialize the map with key-value pairs
    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('a', 1);
    map.insert('b', 2);
    
    // Existing key 'b' is at index 1, inserting it at index 2 (len is now 2)
    let index = 2; // The index equal to the current length
    let key = 'b'; // Key to be moved
    let value = 20; // New value to assign

    // Ensure 'b' is at index 1 before we call insert_before
    assert_eq!(map.get_index_of(&'b'), Some(1));

    // Insert 'b' before index 2 with a new value
    let (new_index, old_value) = map.insert_before(index, key, value);

    // Check the returned index and old value
    assert_eq!(new_index, 1);
    assert_eq!(old_value, Some(2)); // Old value of 'b' which was 2

    // Check the position of 'b' after insert
    assert_eq!(map.get_index_of(&'b'), Some(1)); // Stays at index 1
    assert_eq!(map.len(), 2); // No change in length
}

#[test]
#[should_panic]
fn test_insert_before_with_out_of_bounds_index() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, i32> = IndexMap::new();
    map.insert('a', 1);
    // Attempt to insert with an out-of-bounds index
    let index = 3; // Out of bounds
    let key = 'b';
    let value = 2;

    // This should panic
    map.insert_before(index, key, value);
}

