// Answer 0


#[test]
fn test_or_insert_with_vacant_entry() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::Value;
    
    let mut map = Map::new();
    let result = map.entry("test_key").or_insert_with(|| json!("default_value"));
    
    assert_eq!(result, &mut json!("default_value"));
    assert_eq!(map["test_key"], json!("default_value"));
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use serde_json::json;
    use serde_json::Map;
    use serde_json::Value;

    let mut map = Map::new();
    map.insert("existing_key".to_string(), json!("existing_value"));
    
    {
        let result = map.entry("existing_key").or_insert_with(|| json!("new_value"));
        assert_eq!(result, &mut json!("existing_value"));
    }
    
    assert_eq!(map["existing_key"], json!("existing_value"));
}

#[should_panic]
fn test_or_insert_with_panic_on_empty_key() {
    use serde_json::Map;
    use serde_json::json;
    
    let mut map = Map::new();
    let _result = map.entry("").or_insert_with(|| json!("value"));
}


