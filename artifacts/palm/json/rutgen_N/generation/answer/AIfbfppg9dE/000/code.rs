// Answer 0

#[test]
fn test_map_access_new() {
    use serde_json::Deserializer;
    use serde_json::de::MapAccess;

    let json_data = r#"{}"#;
    let cursor = std::io::Cursor::new(json_data);
    let mut deserializer = Deserializer::from_reader(cursor);
    
    let map_access = MapAccess::new(&mut deserializer);
    
    assert!(map_access.first);
}

