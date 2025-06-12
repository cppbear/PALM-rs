// Answer 0

#[test]
fn test_get_full_existing_key() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    index_map.insert("key2", "value2");
    
    let result = index_map.get_full("key1");
}

#[test]
fn test_get_full_existing_key_at_end() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    index_map.insert("key2", "value2");
    index_map.insert("key3", "value3");
    
    let result = index_map.get_full("key3");
}

#[test]
fn test_get_full_existing_key_middle() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    index_map.insert("key2", "value2");
    index_map.insert("key3", "value3");
    
    let result = index_map.get_full("key2");
}

#[test]
fn test_get_full_multiple_entries_same_key() {
    let mut index_map = IndexMap::new();
    index_map.insert("duplicate_key", "first");
    index_map.insert("duplicate_key", "second");
    
    let result = index_map.get_full("duplicate_key");
}

#[test]
fn test_get_full_with_different_types() {
    let mut index_map: IndexMap<&str, i32> = IndexMap::new();
    index_map.insert("key1", 10);
    index_map.insert("key2", 20);
    
    let result = index_map.get_full("key1");
}

