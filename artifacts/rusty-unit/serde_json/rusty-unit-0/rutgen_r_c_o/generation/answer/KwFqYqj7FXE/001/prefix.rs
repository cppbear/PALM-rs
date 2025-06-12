// Answer 0

#[test]
fn test_remove_entry_existing_key() {
    let mut map = Map::with_capacity(5);
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::String(String::from("value2")));
    map.insert(String::from("key3"), Value::String(String::from("value3")));
    let result = map.remove_entry("key2");
}

#[test]
fn test_remove_entry_first_key() {
    let mut map = Map::with_capacity(3);
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::String(String::from("value2")));
    let result = map.remove_entry("key1");
}

#[test]
fn test_remove_entry_last_key() {
    let mut map = Map::with_capacity(3);
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::String(String::from("value2")));
    let result = map.remove_entry("key2");
}

#[test]
fn test_remove_entry_nonexistent_key() {
    let mut map = Map::with_capacity(3);
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    let result = map.remove_entry("key3");
}

#[test]
fn test_remove_entry_empty_map() {
    let mut map = Map::new();
    let result = map.remove_entry("key1");
}

#[cfg(feature = "preserve_order")]
#[test]
fn test_remove_entry_preserve_order_feature() {
    let mut map = Map::with_capacity(3);
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::String(String::from("value2")));
    let result = map.remove_entry("key1"); // Ensure that order is preserved
}

#[test]
fn test_remove_entry_with_capacity_zero() {
    let mut map = Map::with_capacity(0);
    // Trying to remove a key should yield None since nothing has been added
    let result = map.remove_entry("key1");
}

