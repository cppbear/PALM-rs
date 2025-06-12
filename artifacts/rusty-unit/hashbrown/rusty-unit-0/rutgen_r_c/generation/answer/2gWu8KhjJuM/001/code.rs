// Answer 0

#[test]
fn test_index_existing_key() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("a", "One");
    map.insert("b", "Two");

    assert_eq!(map[&"a"], "One");
    assert_eq!(map[&"b"], "Two");
}

#[should_panic(expected = "no entry found for key")]
#[test]
fn test_index_non_existing_key() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("a", "One");

    let _ = map[&"b"]; // This should panic
}

