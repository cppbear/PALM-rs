// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = Map::new();
    let key = String::from("key1");
    let value = Value::String(String::from("value1"));
    map.insert(key.clone(), value.clone());
    assert_eq!(map.remove(&key), Some(value));
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = Map::new();
    let key = String::from("non_existing_key");
    assert_eq!(map.remove(&key), None);
}

#[test]
fn test_remove_empty_map() {
    let mut map = Map::new();
    let key = String::from("key");
    assert_eq!(map.remove(&key), None);
}

#[test]
fn test_remove_key_multiple_times() {
    let mut map = Map::new();
    let key = String::from("key2");
    let value1 = Value::String(String::from("value1"));
    let value2 = Value::String(String::from("value2"));
    map.insert(key.clone(), value1);
    assert_eq!(map.remove(&key), Some(value1));
    assert_eq!(map.remove(&key), None);
    map.insert(key.clone(), value2);
    assert_eq!(map.remove(&key), Some(value2));
}

#[test]
#[should_panic]
fn test_remove_key_with_invalid_type() {
    let mut map = Map::new();
    let key = String::from("key3");
    map.insert(key.clone(), Value::String(String::from("value3")));
    // Assuming panic when key type does not match
    let _result: Option<Value> = map.remove(&123); // Invalid type as the key is a String
}

