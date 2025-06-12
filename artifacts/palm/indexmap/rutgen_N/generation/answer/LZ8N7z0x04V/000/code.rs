// Answer 0

#[test]
fn test_index_existing_key() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", "value1");
    
    let result = map.index("key1");
    assert_eq!(result, &"value1");
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existing_key() {
    use indexmap::IndexMap;

    let map: IndexMap<&str, &str> = IndexMap::new();
    let _ = map.index("non_existent_key");
}

