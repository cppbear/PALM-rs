// Answer 0

#[test]
fn test_last_entry_valid_non_empty() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_single_element() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_multiple_elements() {
    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_large_map() {
    let mut map = IndexMap::new();
    for i in 0..1000 {
        map.insert(format!("key{}", i), format!("value{}", i));
    }
    let entry = map.last_entry();
}

#[should_panic]
fn test_last_entry_empty_map() {
    let mut map: IndexMap<String, String> = IndexMap::new();
    let entry = map.last_entry();
}

