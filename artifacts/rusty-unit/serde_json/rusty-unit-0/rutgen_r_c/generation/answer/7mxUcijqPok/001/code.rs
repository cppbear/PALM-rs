// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("test".to_string(), Value::Bool(false));

    let entry = Entry::Occupied(OccupiedEntry {
        occupied: map.get_mut("test").unwrap().into(),
    });

    let modified_entry = entry.and_modify(|v| {
        match v {
            Value::Bool(b) => *b = true,
            _ => {}
        }
    });

    if let Entry::Occupied(occupied) = modified_entry {
        assert_eq!(occupied.get(), &Value::Bool(true));
    } else {
        panic!("Expected an occupied entry after modification");
    }
}

