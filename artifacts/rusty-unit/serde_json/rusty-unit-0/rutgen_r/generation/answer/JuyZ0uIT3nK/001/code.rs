// Answer 0

#[test]
fn test_values_mut_with_non_empty_map() {
    use std::collections::HashMap;
    use serde_json::Map;

    let mut map: Map<String, serde_json::Value> = Map::new();
    map.insert("key1".to_string(), serde_json::Value::from(1));
    map.insert("key2".to_string(), serde_json::Value::from(2));

    let mut values_mut_iter = map.values_mut();
    assert_eq!(values_mut_iter.next(), Some(&mut serde_json::Value::from(1)));
    assert_eq!(values_mut_iter.next(), Some(&mut serde_json::Value::from(2)));
}

#[test]
fn test_values_mut_with_empty_map() {
    use std::collections::HashMap;
    use serde_json::Map;

    let mut map: Map<String, serde_json::Value> = Map::new();

    let mut values_mut_iter = map.values_mut();
    assert_eq!(values_mut_iter.next(), None);
}

#[test]
fn test_values_mut_modifies_values() {
    use std::collections::HashMap;
    use serde_json::Map;

    let mut map: Map<String, serde_json::Value> = Map::new();
    map.insert("key1".to_string(), serde_json::Value::from(1));
    map.insert("key2".to_string(), serde_json::Value::from(2));

    {
        let values_mut_iter = map.values_mut();
        for value in values_mut_iter {
            *value = serde_json::Value::from(10);
        }
    }

    assert_eq!(map.get("key1"), Some(&serde_json::Value::from(10)));
    assert_eq!(map.get("key2"), Some(&serde_json::Value::from(10)));
}

