// Answer 0

#[derive(Serialize)]
struct SimpleStruct {
    name: String,
    age: u32,
}

#[test]
fn test_to_string_pretty_with_simple_struct() {
    let value = SimpleStruct {
        name: String::from("Alice"),
        age: 30,
    };
    let result = serde_json::to_string_pretty(&value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert!(string.contains("\"name\": \"Alice\""));
    assert!(string.contains("\"age\": 30"));
}

#[derive(Serialize)]
struct NestedStruct {
    id: u32,
    data: SimpleStruct,
}

#[test]
fn test_to_string_pretty_with_nested_struct() {
    let value = NestedStruct {
        id: 1,
        data: SimpleStruct {
            name: String::from("Bob"),
            age: 25,
        },
    };
    let result = serde_json::to_string_pretty(&value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert!(string.contains("\"id\": 1"));
    assert!(string.contains("\"name\": \"Bob\""));
    assert!(string.contains("\"age\": 25"));
}

#[derive(Serialize)]
struct MapStruct {
    entries: std::collections::HashMap<String, u32>,
}

#[test]
fn test_to_string_pretty_with_map_struct() {
    let mut map = std::collections::HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);
    
    let value = MapStruct { entries: map };
    let result = serde_json::to_string_pretty(&value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert!(string.contains("\"one\": 1"));
    assert!(string.contains("\"two\": 2"));
}

#[derive(Serialize)]
struct EmptyStruct;

#[test]
fn test_to_string_pretty_with_empty_struct() {
    let value = EmptyStruct;
    let result = serde_json::to_string_pretty(&value);
    assert!(result.is_ok());
    let string = result.unwrap();
    assert_eq!(string, "{}");
}

