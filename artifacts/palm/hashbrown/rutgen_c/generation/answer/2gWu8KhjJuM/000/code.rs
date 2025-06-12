// Answer 0

#[test]
fn test_index_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("a", "One");
    map.insert("b", "Two");

    assert_eq!(map["a"], "One");
    assert_eq!(map["b"], "Two");
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_non_existing_key() {
    use hashbrown::HashMap;

    let map: HashMap<&str, &str> = HashMap::new();
    map.insert("a", "One");

    // This should panic as the key "b" does not exist in the map.
    let _ = map["b"];
}

