// Answer 0

#[test]
fn test_iter_mut_empty_map() {
    let mut map: Map<String, Value> = Map::new();
    let mut iter = map.iter_mut();
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_mut_single_entry() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    
    let mut iter = map.iter_mut();
    assert_eq!(iter.len(), 1);
    
    if let Some((key, value)) = iter.next() {
        assert_eq!(key, "key1");
        if let Value::String(ref mut v) = value {
            *v = "modified_value".to_string();
        }
    }
    
    assert_eq!(map.get(&"key1".to_string()).unwrap(), &Value::String("modified_value".to_string()));
}

#[test]
fn test_iter_mut_multiple_entries() {
    let mut map: Map<String, Value> = Map::new();
    map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.insert("key2".to_string(), Value::Number(42.into()));
    
    let mut iter = map.iter_mut();
    assert_eq!(iter.len(), 2);
    
    while let Some((key, value)) = iter.next() {
        match key.as_str() {
            "key1" => {
                if let Value::String(ref mut v) = value {
                    *v = "modified_value1".to_string();
                }
            }
            "key2" => {
                if let Value::Number(ref mut n) = value {
                    *n = 100.into();
                }
            }
            _ => {}
        }
    }
    
    assert_eq!(map.get(&"key1".to_string()).unwrap(), &Value::String("modified_value1".to_string()));
    assert_eq!(map.get(&"key2".to_string()).unwrap(), &Value::Number(100.into()));
}

