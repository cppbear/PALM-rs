// Answer 0

#[test]
fn test_get_mut_existing_key() {
    let mut map = Map::new();
    let key = "key1".to_string();
    let value = Value::Bool(true);
    map.insert(key.clone(), value);
    
    if let Some(v) = map.get_mut(&key) {
        *v = Value::Bool(false); // Modify the value
    }
    
    assert_eq!(map.get(&key), Some(&Value::Bool(false)));
}

#[test]
fn test_get_mut_non_existing_key() {
    let mut map = Map::new();
    let key = "non_existing_key";

    let result = map.get_mut(key);
    assert!(result.is_none());
}

#[test]
fn test_get_mut_empty_map() {
    let mut map = Map::new();
    let key = "some_key";

    let result = map.get_mut(key);
    assert!(result.is_none());
}

#[test]
fn test_get_mut_panic_on_key_with_mismatched_ordering() {
    let mut map = Map::new();
    map.insert("key1".to_string(), Value::Bool(true));

    // Assuming that entering a `&str` that does not match the expected key type will lead to a panic.
    let result = std::panic::catch_unwind(|| {
        map.get_mut(&(1u32 as &dyn std::hash::Hash));
    });

    assert!(result.is_err());
}

#[test]
fn test_get_mut_with_different_borrowed_key() {
    let mut map = Map::new();
    let key = "unique_key".to_string();
    let value = Value::Number(42.into());
    map.insert(key.clone(), value);
    
    let borrowed_key: &String = &key;

    if let Some(v) = map.get_mut(borrowed_key) {
        *v = Value::Number(100.into()); // Modify the value
    }
    
    assert_eq!(map.get(&key), Some(&Value::Number(100.into())));
}

