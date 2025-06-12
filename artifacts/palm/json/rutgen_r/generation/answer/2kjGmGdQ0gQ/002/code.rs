// Answer 0

#[test]
fn test_serialization_of_simple_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct SimpleStruct {
        field1: String,
        field2: i32,
    }

    let value = SimpleStruct {
        field1: "Test".to_string(),
        field2: 42,
    };

    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"field1":"Test","field2":42}"#);
}

#[test]
fn test_serialization_of_nested_structs() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct NestedStruct {
        inner: SimpleStruct,
        flag: bool,
    }

    let value = NestedStruct {
        inner: SimpleStruct {
            field1: "Nested".to_string(),
            field2: 99,
        },
        flag: true,
    };

    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"inner":{"field1":"Nested","field2":99},"flag":true}"#);
}

#[test]
fn test_serialization_of_empty_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct;

    let value = EmptyStruct;

    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "{}");
}

#[test]
fn test_serialization_of_map_with_string_keys() {
    use std::collections::HashMap;
    use serde::Serialize;

    #[derive(Serialize)]
    struct MapStruct {
        data: HashMap<String, i32>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);

    let value = MapStruct { data: map };

    let result = serde_json::to_string(&value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"data":{"key1":10,"key2":20}}"#);
}

