// Answer 0

#[test]
fn test_insert_full_with_non_existing_key() {
    let mut map: IndexMap<String, String, RandomState> = IndexMap::new();
    let (index, old_value) = map.insert_full("key1".to_string(), "value1".to_string());
}

#[test]
fn test_insert_full_with_existing_key() {
    let mut map: IndexMap<String, String, RandomState> = IndexMap::new();
    map.insert_full("key1".to_string(), "value1".to_string());
    let (index, old_value) = map.insert_full("key1".to_string(), "value2".to_string());
}

#[test]
fn test_insert_full_multiple_keys() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    let (index1, old_value1) = map.insert_full("key1".to_string(), 10);
    let (index2, old_value2) = map.insert_full("key2".to_string(), 20);
    let (index3, old_value3) = map.insert_full("key3".to_string(), 30);
}

#[test]
fn test_insert_full_replacing_value() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert_full(1, 100);
    let (index, old_value) = map.insert_full(1, 200);
}

#[test]
fn test_insert_full_large_map() {
    let mut map: IndexMap<u64, u64, RandomState> = IndexMap::new();
    for i in 0..usize::MAX as u64 {
        let (index, old_value) = map.insert_full(i, i * 10);
    }
}

#[test]
fn test_insert_full_edge_case() {
    let mut map: IndexMap<String, String, RandomState> = IndexMap::new();
    let (index1, old_value1) = map.insert_full("key1".to_string(), "value1".to_string());
    let (index2, old_value2) = map.insert_full("key2".to_string(), "value2".to_string());
    let (index3, old_value3) = map.insert_full("key1".to_string(), "value3".to_string());
}

