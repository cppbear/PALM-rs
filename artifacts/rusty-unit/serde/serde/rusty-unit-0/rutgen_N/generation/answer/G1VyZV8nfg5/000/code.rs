// Answer 0

#[derive(Deserialize)]
struct TestStruct {
    value: String,
}

#[derive(Deserialize)]
struct TestEnum {
    #[serde(flatten)]
    inner: TestStruct,
}

#[test]
fn test_deserialize_valid() {
    let json_data = r#"{ "value": "test" }"#;
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<TestStruct, _> = TestStruct::deserialize(deserializer);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, "test");
}

#[test]
fn test_deserialize_invalid() {
    let json_data = r#"{ "wrong_field": "test" }"#;
    let deserializer = serde_json::Deserializer::from_str(json_data);
    let result: Result<TestStruct, _> = TestStruct::deserialize(deserializer);
    assert!(result.is_err());
}

