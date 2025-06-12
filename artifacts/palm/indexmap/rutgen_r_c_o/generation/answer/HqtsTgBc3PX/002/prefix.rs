// Answer 0

#[test]
fn test_shift_remove_full_valid_entry() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    index_map.insert_full(HashValue(1), "key1".to_string(), "value1".to_string());
    index_map.insert_full(HashValue(2), "key2".to_string(), "value2".to_string());
    
    let result = index_map.shift_remove_full(HashValue(1), &"key1".to_string());
}

#[test]
fn test_shift_remove_full_multiple_entries() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    index_map.insert_full(HashValue(1), "key1".to_string(), "value1".to_string());
    index_map.insert_full(HashValue(2), "key2".to_string(), "value2".to_string());
    index_map.insert_full(HashValue(3), "key3".to_string(), "value3".to_string());

    let result = index_map.shift_remove_full(HashValue(2), &"key2".to_string());
}

#[test]
fn test_shift_remove_full_edge_case() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::with_capacity(10);
    index_map.insert_full(HashValue(1), "key1".to_string(), "value1".to_string());

    let result = index_map.shift_remove_full(HashValue(1), &"key1".to_string());
}

#[test]
fn test_shift_remove_full_no_entry() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    index_map.insert_full(HashValue(1), "key1".to_string(), "value1".to_string());

    let result = index_map.shift_remove_full(HashValue(2), &"key2".to_string());
}

