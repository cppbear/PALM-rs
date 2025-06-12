// Answer 0

#[derive(serde::Serialize)]
struct TestStruct {
    field1: String,
    field2: i32,
}

#[test]
fn test_serialize_some_with_valid_struct() {
    let test_value = TestStruct {
        field1: "test".to_string(),
        field2: 42,
    };
    
    let result = serde_json::to_string(&test_value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"field1":"test","field2":42}"#);
}

#[test]
fn test_serialize_some_with_empty_struct() {
    #[derive(serde::Serialize)]
    struct EmptyStruct;

    let empty_value = EmptyStruct;

    let result = serde_json::to_string(&empty_value);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{}"#);
}

#[test]
fn test_serialize_some_with_nested_struct() {
    #[derive(serde::Serialize)]
    struct NestedStruct {
        inner: TestStruct,
    }

    let nested_value = NestedStruct {
        inner: TestStruct {
            field1: "nested".to_string(),
            field2: 10,
        },
    };

    let result = serde_json::to_string(&nested_value);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r#"{"inner":{"field1":"nested","field2":10}}"#);
}

#[test]
#[should_panic] // Intentionally cause a panic due to serialization.
fn test_serialize_some_with_non_serializable_type() {
    struct NonSerializableStruct {
        field: std::cell::RefCell<i32>,
    }

    let non_serializable_value = NonSerializableStruct {
        field: std::cell::RefCell::new(1),
    };

    let _ = serde_json::to_string(&non_serializable_value);
}

