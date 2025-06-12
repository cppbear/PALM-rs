// Answer 0

#[test]
fn test_shift_remove_index_with_valid_index() {
    // Setup
    let mut map = IndexMapCore::<usize, String>::new();
    map.entries.push(Bucket { key: 1, value: "One".to_string() });
    map.entries.push(Bucket { key: 2, value: "Two".to_string() });
    map.entries.push(Bucket { key: 3, value: "Three".to_string() });

    // Action
    let removed = map.shift_remove_index(1); // Removing entry at index 1

    // Assert
    assert_eq!(removed, Some((2, "Two".to_string())));
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].value, "One".to_string());
    assert_eq!(map.entries[1].value, "Three".to_string());
}

#[test]
fn test_shift_remove_index_with_empty_map() {
    // Setup
    let mut map = IndexMapCore::<usize, String>::new();

    // Action
    let removed = map.shift_remove_index(0); // Trying to remove from an empty map

    // Assert
    assert_eq!(removed, None);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_shift_remove_index_with_out_of_bounds_index() {
    // Setup
    let mut map = IndexMapCore::<usize, String>::new();
    map.entries.push(Bucket { key: 1, value: "One".to_string() });
    
    // Action
    let _ = map.shift_remove_index(1); // Trying to remove index 1 which is out of bounds
}

#[test]
fn test_shift_remove_index_removes_correct_entry_and_updates_indices() {
    // Setup
    let mut map = IndexMapCore::<usize, String>::new();
    map.entries.push(Bucket { key: 1, value: "One".to_string() });
    map.entries.push(Bucket { key: 2, value: "Two".to_string() });
    map.entries.push(Bucket { key: 3, value: "Three".to_string() });

    // Action
    let removed = map.shift_remove_index(0); // Removing entry at index 0

    // Assert
    assert_eq!(removed, Some((1, "One".to_string())));
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[0].value, "Two".to_string());
    assert_eq!(map.entries[1].value, "Three".to_string());
}

