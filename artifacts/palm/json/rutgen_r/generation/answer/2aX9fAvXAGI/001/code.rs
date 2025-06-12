// Answer 0

#[test]
fn test_serialize_some_err_key_must_be_a_string() {
    struct NonSerializableStruct;

    let value = NonSerializableStruct;

    let result: Result<String> = serialize_some(&value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "key must be a string");
}

