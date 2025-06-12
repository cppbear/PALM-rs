// Answer 0

#[test]
fn test_into_values_with_empty_map() {
    let map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let into_values = serde_json::value::Map::from(map).into_values();
    let values: Vec<i32> = into_values.iter.collect();
    assert_eq!(values, Vec::<i32>::new());
}

#[test]
fn test_into_values_with_single_element_map() {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    map.insert("key1".to_string(), 42);
    let into_values = serde_json::value::Map::from(map).into_values();
    let values: Vec<i32> = into_values.iter.collect();
    assert_eq!(values, vec![42]);
}

#[test]
fn test_into_values_with_multiple_element_map() {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    let into_values = serde_json::value::Map::from(map).into_values();
    let values: Vec<i32> = into_values.iter.collect();
    assert_eq!(values, vec![1, 2]);
}

#[test]
fn test_into_values_with_varied_values_map() {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    map.insert("key1".to_string(), -1);
    map.insert("key2".to_string(), 0);
    map.insert("key3".to_string(), 100);
    let into_values = serde_json::value::Map::from(map).into_values();
    let values: Vec<i32> = into_values.iter.collect();
    assert_eq!(values, vec![-1, 0, 100]);
}

