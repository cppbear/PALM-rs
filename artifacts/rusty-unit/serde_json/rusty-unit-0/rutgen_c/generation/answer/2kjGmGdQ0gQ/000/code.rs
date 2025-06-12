// Answer 0

#[test]
fn test_to_string_with_valid_serializable() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    let value = TestStruct { 
        field1: "test".to_string(), 
        field2: 42 
    };

    let result = serde_json::to_string(&value).unwrap();
    assert_eq!(result, r#"{"field1":"test","field2":42}"#);
}

#[test]
fn test_to_string_with_empty_struct() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct;

    let value = EmptyStruct;

    let result = serde_json::to_string(&value).unwrap();
    assert_eq!(result, r#"{}"#);
}

#[should_panic]
fn test_to_string_with_non_serializable() {
    use serde::Serialize;
    struct NonSerializable;

    let value = NonSerializable;

    // This should panic because NonSerializable does not implement Serialize
    let _result = serde_json::to_string(&value).unwrap();
}

#[test]
fn test_to_string_with_complex_structure() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct ComplexStruct {
        name: String,
        age: u32,
        friends: Vec<String>,
    }

    let value = ComplexStruct { 
        name: "Alice".to_string(), 
        age: 30, 
        friends: vec!["Bob".to_string(), "Charlie".to_string()]
    };

    let result = serde_json::to_string(&value).unwrap();
    assert_eq!(result, r#"{"name":"Alice","age":30,"friends":["Bob","Charlie"]}"#);
}

