// Answer 0

#[test]
fn test_invalid_type_null() {
    let value = Value::Null;
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

#[test]
fn test_invalid_type_bool() {
    let value = Value::Bool(true);
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

#[test]
fn test_invalid_type_number() {
    let value = Value::Number(Number { n: 42 });
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

#[test]
fn test_invalid_type_string() {
    let value = Value::String("test".to_string());
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

#[test]
fn test_invalid_type_array() {
    let value = Value::Array(vec![]);
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

#[test]
fn test_invalid_type_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let expected = mock_expected();
    let result: serde::de::Error = value.invalid_type(&expected);
    assert!(matches!(result, serde::de::Error::InvalidType { .. }));
}

fn mock_expected() -> impl Expected {
    struct MockExpected;
    impl Expected for MockExpected {}
    MockExpected
}

