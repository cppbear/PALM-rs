// Answer 0

#[test]
fn test_insert_full_occupied() {
    #[derive(Eq, PartialEq)]
    struct Key(usize);
    
    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(123);
    let key = Key(1);
    let value = 42;

    // Insert an initial value to create an occupied entry.
    index_map.insert_full(hash_value, key, value);

    // Now insert with the same key and expect to get back the previous value.
    let new_value = 43;
    let (index, previous_value) = index_map.insert_full(hash_value, key, new_value);

    assert_eq!(index, 0); // Should be at index 0
    assert_eq!(previous_value, Some(value)); // Should return the old value
}

#[test]
fn test_insert_full_vacant() {
    #[derive(Eq, PartialEq)]
    struct Key(usize);

    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(456);
    let key = Key(2);
    let value = 24;

    // Insert for the first time, it should be a vacant entry.
    let (index, previous_value) = index_map.insert_full(hash_value, key, value);

    assert_eq!(index, 0); // The first insertion goes to index 0
    assert_eq!(previous_value, None); // There was no previous value
}

#[test]
fn test_insert_full_multiple_collision() {
    #[derive(Eq, PartialEq)]
    struct Key(usize);

    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(789);
    let key1 = Key(3);
    let key2 = Key(4);
    let value1 = 12;
    let value2 = 34;

    // Insert first key-value pair
    index_map.insert_full(hash_value, key1, value1);

    // Insert second key-value pair generating a collision with hash_value
    let (index, previous_value) = index_map.insert_full(hash_value, key2, value2);

    assert_eq!(index, 1); // Second entry at index 1
    assert_eq!(previous_value, None); // No previous value for key2
}

#[test]
fn test_insert_full_multiple_updates() {
    #[derive(Eq, PartialEq)]
    struct Key(usize);

    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(321);
    let key = Key(5);
    let value1 = 100;
    let value2 = 200;

    // First insertion with value1
    index_map.insert_full(hash_value, key, value1);

    // Second insertion updating the same key with value2
    let (index, previous_value) = index_map.insert_full(hash_value, key, value2);

    assert_eq!(index, 0); // Should be in index 0
    assert_eq!(previous_value, Some(value1)); // Previous value should be value1
}

