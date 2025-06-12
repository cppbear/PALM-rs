// Answer 0

#[test]
fn test_as_entries_mut() {
    // Define a basic structure for the test
    let mut index_map: IndexMapCore<i32, String> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "Two".to_string() },
        ],
    };

    // Get mutable entries
    let entries_mut = index_map.as_entries_mut();

    // Modify the first entry
    entries_mut[0].value = "Changed".to_string();

    // Assert the value has changed
    assert_eq!(index_map.entries[0].value, "Changed".to_string());
}

#[test]
fn test_as_entries_mut_empty() {
    // Test with an empty entries vector
    let mut index_map: IndexMapCore<i32, String> = IndexMapCore {
        indices: hash_table::HashTable::new(),
        entries: vec![],
    };

    // Get mutable entries
    let entries_mut = index_map.as_entries_mut();

    // Ensure we can access the mutable slice even if it's empty
    assert!(entries_mut.is_empty());
}

