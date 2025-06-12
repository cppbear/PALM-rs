// Answer 0

#[derive(Serialize)]
struct SimpleStruct {
    name: String,
    value: i32,
}

#[test]
fn test_to_string_pretty_valid_structure() {
    let input = SimpleStruct {
        name: "Test".to_string(),
        value: 42,
    };
    let result = serde_json::to_string_pretty(&input).unwrap();
    assert!(result.contains("Test"));
    assert!(result.contains("42"));
}

#[test]
fn test_to_string_pretty_empty_structure() {
    #[derive(Serialize)]
    struct EmptyStruct;

    let input = EmptyStruct;
    let result = serde_json::to_string_pretty(&input).unwrap();
    assert_eq!(result, "{}");
}

#[derive(Serialize)]
struct StructWithMap {
    data: std::collections::HashMap<String, i32>,
}

#[test]
fn test_to_string_pretty_struct_with_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);
    
    let input = StructWithMap { data: map };
    let result = serde_json::to_string_pretty(&input).unwrap();
    
    assert!(result.contains("\"key1\": 1"));
    assert!(result.contains("\"key2\": 2"));
}

#[test]
fn test_to_string_pretty_vec_of_structs() {
    let input = vec![
        SimpleStruct { name: "A".to_string(), value: 1 },
        SimpleStruct { name: "B".to_string(), value: 2 },
    ];
    
    let result = serde_json::to_string_pretty(&input).unwrap();
    
    assert!(result.contains("\"name\": \"A\""));
    assert!(result.contains("\"value\": 1"));
    assert!(result.contains("\"name\": \"B\""));
    assert!(result.contains("\"value\": 2"));
}

