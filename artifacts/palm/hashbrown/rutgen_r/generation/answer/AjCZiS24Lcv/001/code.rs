// Answer 0

#[test]
fn test_hashmap_new_empty() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_hashmap_new_non_empty_after_insert() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 1);
    assert_eq!(map.len(), 1);
    assert!(map.capacity() > 0); // capacity should be greater than 0 after insertion
}

#[test]
fn test_hashmap_new_with_same_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 1);
    map.insert("key1", 2); // this will update the value for "key1"
    assert_eq!(map.len(), 1);
    assert_eq!(map["key1"], 2);
}

#[should_panic]
fn test_hashmap_new_panic_on_invalid_access() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    let _value = map["non_existent_key"]; // This should panic
}

