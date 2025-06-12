// Answer 0

#[test]
fn test_contains_key_existing_key() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let result = map.contains_key(&"key1");
}

#[test]
fn test_contains_key_non_existing_key() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let result = map.contains_key(&"non_existing_key");
}

#[test]
fn test_contains_key_empty_key() {
    let mut map = IndexMap::new();
    let result = map.contains_key(&"");
}

#[test]
fn test_contains_key_duplicate_keys() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key1", "value2");
    let result = map.contains_key(&"key1");
}

#[test]
fn test_contains_key_large_map() {
    let mut map = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, i * 2);
    }
    let result = map.contains_key(&500);
}

#[test]
fn test_contains_key_with_different_type() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let result = map.contains_key(&String::from("key1"));
}

#[test]
fn test_contains_key_null_key() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let result = map.contains_key::<&str>(core::ptr::null());
}

#[test]
fn test_contains_key_after_removal() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.remove("key1");
    let result = map.contains_key(&"key1");
}

