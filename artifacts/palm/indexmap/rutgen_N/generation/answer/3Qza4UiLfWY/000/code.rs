// Answer 0

#[test]
fn test_index_mut_existing_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("key1", 10);

    let value = map.index_mut("key1");
    *value += 5;

    assert_eq!(map["key1"], 15);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.insert("key1", 10);

    let _value = map.index_mut("key2"); // This should panic
}

