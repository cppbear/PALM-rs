// Answer 0

#[test]
fn test_shift_remove_entry_existing_key() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);
    let result = map.shift_remove_entry(&"key2".to_string());
}

#[test]
fn test_shift_remove_entry_multiple_keys() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("keyA".to_string(), 10);
    map.insert("keyB".to_string(), 20);
    map.insert("keyC".to_string(), 30);
    let result = map.shift_remove_entry(&"keyA".to_string());
}

#[test]
fn test_shift_remove_entry_last_key() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("lastKey".to_string(), 100);
    let result = map.shift_remove_entry(&"lastKey".to_string());
}

#[test]
fn test_shift_remove_entry_non_existent_key() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("existingKey".to_string(), 5);
    let result = map.shift_remove_entry(&"nonExistentKey".to_string());
}

#[test]
fn test_shift_remove_entry_after_insertion() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("keyX".to_string(), 10);
    map.insert("keyY".to_string(), 20);
    map.insert("keyZ".to_string(), 30);
    map.insert("keyW".to_string(), 40);
    let result = map.shift_remove_entry(&"keyY".to_string());
}

