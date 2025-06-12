// Answer 0

#[test]
fn test_iter_mut_empty() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    let mut iter = map.iter_mut();
    assert!(iter.cursor.is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1".to_string());
    let mut iter = map.iter_mut();
    
    assert!(iter.cursor.is_some());
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "Key1");
        assert_eq!(value, "Value1");
        value.push_str("-boop");
        assert_eq!(value, "Value1-boop");
    } else {
        panic!("Expected an entry");
    }
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1".to_string());
    map.append("Key1", "Value1.1".to_string());
    map.insert("Key2", "Value2".to_string());

    let mut iter = map.iter_mut();
    
    let mut keys: Vec<&str> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    
    while let Some((key, value)) = iter.next() {
        keys.push(key);
        values.push(value.clone());
        value.push_str("-boop");
    }

    assert_eq!(keys.len(), 3);
    assert_eq!(keys, vec!["Key1", "Key1", "Key2"]);
    assert_eq!(values, vec!["Value1", "Value1.1", "Value2"]);
}

#[test]
fn test_iter_mut_after_clear() {
    let mut map: HeaderMap<String> = HeaderMap::with_capacity(10);
    map.insert("Key1", "Value1".to_string());
    map.clear();
    
    let mut iter = map.iter_mut();
    assert!(iter.cursor.is_none());
}

