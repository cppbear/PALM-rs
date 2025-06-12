// Answer 0

#[derive(Serialize)]
struct SimpleStruct {
    name: String,
    age: u32,
}

#[test]
fn test_to_string_with_simple_struct() {
    let value = SimpleStruct {
        name: "Alice".to_string(),
        age: 30,
    };
    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{\"name\":\"Alice\",\"age\":30}");
}

#[derive(Serialize)]
struct NestedStruct {
    inner: SimpleStruct,
    is_active: bool,
}

#[test]
fn test_to_string_with_nested_struct() {
    let value = NestedStruct {
        inner: SimpleStruct {
            name: "Bob".to_string(),
            age: 25,
        },
        is_active: true,
    };
    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{\"inner\":{\"name\":\"Bob\",\"age\":25},\"is_active\":true}");
}

#[derive(Serialize)]
struct StructWithVec {
    numbers: Vec<u32>,
}

#[test]
fn test_to_string_with_struct_containing_vec() {
    let value = StructWithVec { numbers: vec![1, 2, 3] };
    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{\"numbers\":[1,2,3]}");
}

#[derive(Serialize)]
struct StructWithMap {
    mappings: std::collections::HashMap<String, u32>,
}

#[test]
fn test_to_string_with_struct_containing_map() {
    let mut map = std::collections::HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    
    let value = StructWithMap { mappings: map };
    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{\"mappings\":{\"one\":1,\"two\":2}}");
}

#[derive(Serialize)]
struct StructWithOptionalField {
    name: String,
    age: Option<u32>,
}

#[test]
fn test_to_string_with_optional_field() {
    let value = StructWithOptionalField {
        name: "Charlie".to_string(),
        age: None,
    };
    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{\"name\":\"Charlie\",\"age\":null}");
}

