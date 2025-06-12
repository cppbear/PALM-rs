// Answer 0

#[test]
fn test_index_mut_valid_key() {
    let mut map = indexmap::IndexMap::new();
    map.insert("key1", 42);
    let value = map.index_mut("key1");
    *value += 1;
    assert_eq!(*value, 43);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_invalid_key() {
    let mut map = indexmap::IndexMap::new();
    map.insert("key1", 42);
    let _value = map.index_mut("key2"); // This should panic
}

#[test]
fn test_index_mut_updates_value() {
    let mut map = indexmap::IndexMap::new();
    map.insert("key1", 50);
    {
        let value = map.index_mut("key1");
        *value += 10;
    }
    assert_eq!(map.get("key1").unwrap(), &60);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_empty_map() {
    let mut map = indexmap::IndexMap::new();
    let _value = map.index_mut("nonexistent_key"); // This should panic
}

