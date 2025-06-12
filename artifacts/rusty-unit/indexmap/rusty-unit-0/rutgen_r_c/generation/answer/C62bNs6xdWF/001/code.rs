// Answer 0

#[test]
fn test_swap_remove_index_with_valid_index() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.push_entry(HashValue::new(0), 1, 10);
    index_map.push_entry(HashValue::new(1), 2, 20);
    index_map.push_entry(HashValue::new(2), 3, 30);

    let result = index_map.swap_remove_index(1);
    assert_eq!(result, Some((2, 20)));

    // Verify that the expected entry is removed
    assert_eq!(index_map.len(), 2);
    assert_eq!(index_map.entries[1].0, 3); // The last element should shift to index 1
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_remove_index_with_out_of_bounds_index() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.push_entry(HashValue::new(0), 1, 10);
    index_map.push_entry(HashValue::new(1), 2, 20);

    // Trying to remove index 3 which is out of bounds
    let _ = index_map.swap_remove_index(3);
}

#[test]
fn test_swap_remove_index_with_empty_map() {
    let mut index_map = IndexMapCore::<usize, usize>::new();

    // Swap remove on an empty map should return None
    let result = index_map.swap_remove_index(0);
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_index_removes_last_element() {
    let mut index_map = IndexMapCore::<usize, usize>::new();
    index_map.push_entry(HashValue::new(0), 1, 10);

    // Removing the last element
    let result = index_map.swap_remove_index(0);
    assert_eq!(result, Some((1, 10))); // Check the correct entry is returned
    assert_eq!(index_map.len(), 0); // Should be empty afterwards
}

