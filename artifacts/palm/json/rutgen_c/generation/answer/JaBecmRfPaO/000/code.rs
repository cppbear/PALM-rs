// Answer 0

#[test]
fn test_get_mut_existing_key() {
    let mut map = Map::new();
    let key = String::from("key1");
    let value = Value::String(String::from("value1"));
    map.insert(key.clone(), value);

    if let Some(val_ref) = map.get_mut(&key) {
        *val_ref = Value::String(String::from("updated_value"));
    }

    assert_eq!(map.get(&key), Some(&Value::String(String::from("updated_value"))));
}

#[test]
fn test_get_mut_non_existing_key() {
    let mut map = Map::new();
    let key = String::from("non_existing_key");

    let result = map.get_mut(&key);
    assert!(result.is_none());
}

#[test]
fn test_get_mut_after_insert() {
    let mut map = Map::new();
    let key = String::from("key2");
    let value = Value::Number(42.into());
    map.insert(key.clone(), value);

    if let Some(val_ref) = map.get_mut(&key) {
        *val_ref = Value::Number(100.into());
    }

    assert_eq!(map.get(&key), Some(&Value::Number(100.into())));
}

