// Answer 0

#[test]
fn test_get_key_value_existing_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    let result = map.get_key_value(&1);
    assert_eq!(result, Some((&1, &"one")));
}

#[test]
fn test_get_key_value_non_existing_key() {
    use indexmap::IndexMap;

    let mut map: IndexMap<i32, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");

    let result = map.get_key_value(&3);
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, &str> = IndexMap::new();

    let result = map.get_key_value(&1);
    assert_eq!(result, None);
}

#[test]
fn test_get_key_value_key_as_reference() {
    use indexmap::IndexMap;

    let mut map: IndexMap<String, i32> = IndexMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);

    let result = map.get_key_value(&"one");
    assert_eq!(result, Some((&"one".to_string(), &1)));
}

