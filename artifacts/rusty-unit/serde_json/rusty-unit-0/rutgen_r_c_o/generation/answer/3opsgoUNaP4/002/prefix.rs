// Answer 0

#[test]
fn test_key_vacant_entry() {
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    
    let vacant_entry = map.entry(String::from("test_key"));
    let key = vacant_entry.key();
}

#[test]
fn test_key_occupied_entry() {
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    
    map.insert(String::from("occupied_key"), Value::String(String::from("occupied_value")));
    let occupied_entry = map.entry(String::from("occupied_key"));
    let key = occupied_entry.key();
}

