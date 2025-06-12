// Answer 0

#[test]
fn test_values_mut_empty() {
    let mut map = Map::new();
    let mut values_iter = map.values_mut();
    assert!(values_iter.iter.next().is_none());
}

#[test]
fn test_values_mut_single() {
    let mut map = Map::new();
    map.insert("key".to_string(), Value::Number(serde_json::Number::from(1)));
    let mut values_iter = map.values_mut();
    if let Some(value) = values_iter.iter.next() {
        match value {
            Value::Number(n) => assert_eq!(n, &serde_json::Number::from(1)),
            _ => panic!("Expected a Number value"),
        }
    } else {
        panic!("Expected an element but found none");
    }
}

#[test]
fn test_values_mut_multiple() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Number(serde_json::Number::from(1)));
    map.insert("key2".to_string(), Value::String("value".to_string()));
    let mut values_iter = map.values_mut();

    let values: Vec<&Value> = values_iter.iter.collect();
    assert_eq!(values.len(), 2);
    assert!(matches!(values[0], Value::Number(_)));
    assert!(matches!(values[1], Value::String(_)));
}

#[test]
fn test_values_mut_modification() {
    let mut map = Map::new();
    map.insert("key".to_string(), Value::Number(serde_json::Number::from(1)));
    {
        let mut values_iter = map.values_mut();
        if let Some(value) = values_iter.iter.next() {
            *value = Value::Number(serde_json::Number::from(2));
        }
    }
    let modified_value = map.get(&"key".to_string()).unwrap();
    assert!(matches!(modified_value, Value::Number(n) if *n == serde_json::Number::from(2)));
}

