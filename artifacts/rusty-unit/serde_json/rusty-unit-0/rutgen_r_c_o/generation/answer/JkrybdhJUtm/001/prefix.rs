// Answer 0

#[test]
fn test_or_insert_with_occupied_entry_bool() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key1").unwrap() });
    entry.or_insert_with(|| Value::Bool(false));
}

#[test]
fn test_or_insert_with_occupied_entry_number() {
    let mut map = serde_json::Map::new();
    map.insert("key2".to_string(), Value::Number(Number::from(42)));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key2").unwrap() });
    entry.or_insert_with(|| Value::Number(Number::from(24)));
}

#[test]
fn test_or_insert_with_occupied_entry_string() {
    let mut map = serde_json::Map::new();
    map.insert("key3".to_string(), Value::String("hello".to_string()));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key3").unwrap() });
    entry.or_insert_with(|| Value::String("world".to_string()));
}

#[test]
fn test_or_insert_with_occupied_entry_array() {
    let mut map = serde_json::Map::new();
    map.insert("key4".to_string(), Value::Array(vec![Value::String("item1".to_string())]));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key4").unwrap() });
    entry.or_insert_with(|| Value::Array(vec![Value::String("item2".to_string())]));
}

#[test]
fn test_or_insert_with_occupied_entry_object() {
    let mut map = serde_json::Map::new();
    let mut nested_map = serde_json::Map::new();
    nested_map.insert("nested_key".to_string(), Value::Bool(true));
    map.insert("key5".to_string(), Value::Object(nested_map));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key5").unwrap() });
    entry.or_insert_with(|| Value::Object(serde_json::Map::new()));
}

#[test]
fn test_or_insert_with_large_key() {
    let mut map = serde_json::Map::new();
    let key = "a".repeat(100);
    map.insert(key.clone(), Value::Bool(true));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut(&key).unwrap() });
    entry.or_insert_with(|| Value::Bool(false));
}

