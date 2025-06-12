// Answer 0

#[test]
fn test_is_empty_with_no_elements() {
    let map: Map<String, Value> = Map::new();
    map.is_empty();
}

#[test]
fn test_is_empty_with_one_element() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::Null);
    map.is_empty();
}

