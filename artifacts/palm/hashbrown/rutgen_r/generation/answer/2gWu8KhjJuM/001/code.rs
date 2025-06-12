// Answer 0

#[test]
fn test_index_existing_key() {
    use hashbrown::HashMap;

    let map: HashMap<_, _> = [("key1", "value1"), ("key2", "value2")].into();
    assert_eq!(map.index(&"key1"), &"value1");
    assert_eq!(map.index(&"key2"), &"value2");
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existing_key() {
    use hashbrown::HashMap;

    let map: HashMap<_, _> = [("key1", "value1")].into();
    let _ = map.index(&"key2");
}

#[test]
fn test_index_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<&str, &str> = HashMap::new();
    assert!(std::panic::catch_unwind(|| {
        let _ = map.index(&"nonexistent");
    }).is_err());
}

