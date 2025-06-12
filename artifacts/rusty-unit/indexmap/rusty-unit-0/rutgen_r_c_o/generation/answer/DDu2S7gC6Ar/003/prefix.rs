// Answer 0

#[test]
fn test_replace_full_occupied_case() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    let hash = HashValue(1);
    index_map.replace_full(hash, "key1".to_string(), "value1".to_string());
    let result = index_map.replace_full(hash, "key1_updated".to_string(), "value1_updated".to_string());
}

#[test]
fn test_replace_full_occupied_case_multiple() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    let hash1 = HashValue(2);
    index_map.replace_full(hash1, "key2".to_string(), "value2".to_string());
    let hash2 = HashValue(3);
    index_map.replace_full(hash2, "key3".to_string(), "value3".to_string());
    let result1 = index_map.replace_full(hash1, "key2_updated".to_string(), "value2_updated".to_string());
    let result2 = index_map.replace_full(hash2, "key3_updated".to_string(), "value3_updated".to_string());
}

#[test]
fn test_replace_full_vacant_case() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::new();
    let hash = HashValue(4);
    let result = index_map.replace_full(hash, "new_key".to_string(), "new_value".to_string());
}

#[test]
fn test_replace_full_edge_case_high_hash() {
    let mut index_map: IndexMapCore<String, String> = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    let hash = HashValue(usize::MAX as usize);
    let result = index_map.replace_full(hash, "edge_key".to_string(), "edge_value".to_string());
}

