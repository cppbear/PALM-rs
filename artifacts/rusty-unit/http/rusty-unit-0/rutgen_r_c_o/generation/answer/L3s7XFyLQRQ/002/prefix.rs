// Answer 0

#[test]
fn test_remove_entry() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::new("host");
    let value = HeaderValue::from("world");

    map.insert(key.clone(), value.clone());

    let entry = map.entry("host").unwrap();
    let (returned_key, returned_value) = entry.remove_entry();

    assert_eq!(returned_key, key);
    assert_eq!(returned_value, value);
    assert!(!map.contains_key("host"));
}

#[test]
fn test_remove_entry_with_multiple_values() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::new("host");
    let value1 = HeaderValue::from("world");
    let value2 = HeaderValue::from("earth");

    // Insert multiple values for the same key
    map.append(key.clone(), value1.clone());
    map.append(key.clone(), value2.clone());

    let entry = map.entry("host").unwrap();
    let (returned_key, mut drain) = entry.remove_entry_mult();
    let values: Vec<_> = drain.collect();

    assert_eq!(returned_key, key);
    assert_eq!(values, vec![value1, value2]);
    assert!(!map.contains_key("host"));
}

#[test]
fn test_remove_entry_with_existing_links() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::new("host");
    let value = HeaderValue::from("world");

    map.insert(key.clone(), value.clone());

    let entry = map.entry("host").unwrap();
    
    // Simulate creating links in the entry
    entry.append(HeaderValue::from("additional_value"));

    let (returned_key, returned_value) = entry.remove_entry();

    assert_eq!(returned_key, key);
    assert_eq!(returned_value, value);
    assert!(!map.contains_key("host"));
}

#[test]
#[should_panic]
fn test_remove_entry_invalid_index() {
    // Attempt to create an entry with an invalid index in an empty map
    let mut empty_map: HeaderMap<HeaderValue> = HeaderMap::with_capacity(0);
    let empty_entry = empty_map.entry("invalid_key"); // This will not be occupied
    let _ = empty_entry.unwrap_err();
} 

#[test]
fn test_remove_entry_with_edge_index() {
    let mut map = HeaderMap::with_capacity(10);
    let key = HeaderName::new("edge_case");
    let value = HeaderValue::from("value");

    map.insert(key.clone(), value.clone());
    let entry = map.entry("edge_case").unwrap();
    let (returned_key, returned_value) = entry.remove_entry();

    assert_eq!(returned_key, key);
    assert_eq!(returned_value, value);
    assert!(!map.contains_key("edge_case"));
}

