// Answer 0

#[test]
fn test_last_mut_empty() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.last_mut();
}

#[test]
fn test_last_mut_one_entry() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("key1".to_string(), 1);
    map.last_mut();
}

#[test]
fn test_last_mut_multiple_entries() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    map.insert("key3".to_string(), 3);
    map.last_mut();
}

#[test]
fn test_last_mut_large_entry_count() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, i * 2);
    }
    map.last_mut();
}

