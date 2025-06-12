// Answer 0

#[test]
fn test_sort_keys_empty_map() {
    let mut map = Map::new();
    map.sort_keys();
    assert!(map.is_empty());
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_ordered_map() {
    let mut map = Map::new();
    map.insert("b".to_string(), Value::Number(2.0));
    map.insert("a".to_string(), Value::Number(1.0));
    map.insert("c".to_string(), Value::Number(3.0));
    
    map.sort_keys();
    
    let keys: Vec<String> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_unordered_map() {
    let mut map = Map::new();
    map.insert("z".to_string(), Value::String("last".to_string()));
    map.insert("a".to_string(), Value::String("first".to_string()));
    map.insert("m".to_string(), Value::String("middle".to_string()));
    
    map.sort_keys();
    
    let keys: Vec<String> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["a".to_string(), "m".to_string(), "z".to_string()]);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_sort_keys_no_op() {
    let mut map = Map::new();
    map.insert("baz".to_string(), Value::Number(5.0));
    map.insert("foo".to_string(), Value::Number(3.0));
    
    let original_keys: Vec<String> = map.keys().cloned().collect();
    map.sort_keys();
    
    let keys_after_sort: Vec<String> = map.keys().cloned().collect();
    assert_eq!(original_keys, keys_after_sort);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_nested_map() {
    let mut map = Map::new();
    let nested_map = Map::new();
    map.insert("b".to_string(), Value::Object(nested_map));
    map.insert("a".to_string(), Value::Number(1.0));
    
    map.sort_keys();
    
    let keys: Vec<String> = map.keys().cloned().collect();
    assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
}

#[test]
#[cfg(feature = "preserve_order")]
fn test_sort_keys_large_map() {
    let mut map = Map::new();
    for i in (0..1000).rev() {
        map.insert(format!("key{}", i), Value::Number(i as f64));
    }
    
    map.sort_keys();
    
    let keys: Vec<String> = map.keys().cloned().collect();
    assert_eq!(keys, (0..1000).map(|i| format!("key{}", i)).collect::<Vec<_>>());
}

