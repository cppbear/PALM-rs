// Answer 0

#[test]
fn test_serialize_some_with_valid_serializable() {
    use serde_json::{Value, to_value};
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        field: String,
    }

    let test_value = TestStruct {
        field: String::from("test"),
    };

    let result: Result<Value> = {
        let serializer = serde_json::Serializer::new(Vec::new());
        serializer.serialize_some(&test_value)
    };

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), to_value(test_value).unwrap());
}

#[test]
fn test_serialize_some_with_empty_struct() {
    use serde_json::{Value, to_value};
    use serde::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct;

    let empty_value = EmptyStruct;

    let result: Result<Value> = {
        let serializer = serde_json::Serializer::new(Vec::new());
        serializer.serialize_some(&empty_value)
    };

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), to_value(empty_value).unwrap());
}

#[test]
#[should_panic]
fn test_serialize_some_with_non_serializable() {
    use serde::Serialize;

    struct NonSerializableStruct;

    let non_serializable_value = NonSerializableStruct;

    {
        let serializer = serde_json::Serializer::new(Vec::new());
        let _result: Result<Value> = serializer.serialize_some(&non_serializable_value);
    }
}

