// Answer 0

#[test]
fn test_insert_full_vacant_entry() {
    // Initialize the IndexMapCore with usize as K and String as V
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();

    // Create a HashValue for the test
    let hash_value = HashValue(42);
    let key = 1;
    let value = String::from("value1");

    // Insert a key-value pair which should be a vacant entry
    let (index, previous_value) = index_map.insert_full(hash_value, key, value.clone());

    // Verify the index at which the item was inserted
    assert_eq!(index, 0);
    // Verify that previously there was no value at this index
    assert_eq!(previous_value, None);
    // Verify the item was actually inserted
    assert_eq!(index_map.entries.len(), 1);
    assert_eq!(index_map.entries[0].key, key);
    assert_eq!(index_map.entries[0].value, value);
}

#[test]
fn test_insert_full_occupied_entry() {
    // Initialize the IndexMapCore with usize as K and String as V
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();

    // Create a HashValue for two test entries
    let hash_value_1 = HashValue(42);
    let key_1 = 1;
    let value_1 = String::from("value1");

    let hash_value_2 = HashValue(42);
    let key_2 = 1;
    let value_2 = String::from("value2");

    // Insert first key-value pair
    index_map.insert_full(hash_value_1, key_1, value_1);

    // Insert another key-value pair with the same key which should now be occupied
    let (index, previous_value) = index_map.insert_full(hash_value_2, key_2, value_2.clone());

    // Verify the index at which the item was updated
    assert_eq!(index, 0);
    // Verify that the previous value was returned
    assert_eq!(previous_value, Some(String::from("value1")));
    // Verify that the value has been updated to the new value
    assert_eq!(index_map.entries[0].value, value_2);
}

