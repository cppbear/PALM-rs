// Answer 0

#[test]
fn test_new_with_non_empty_slice() {
    let values: &[serde_json::Value] = &[
        serde_json::Value::String("test".to_string()),
        serde_json::Value::Number(serde_json::Number::from(42)),
        serde_json::Value::Bool(true),
    ];
    let deserializer = SeqRefDeserializer::new(values);
    assert_eq!(deserializer.iter.as_slice(), values);
}

#[test]
fn test_new_with_empty_slice() {
    let values: &[serde_json::Value] = &[];
    let deserializer = SeqRefDeserializer::new(values);
    assert_eq!(deserializer.iter.as_slice(), values);
}

#[test]
fn test_new_with_single_value() {
    let values: &[serde_json::Value] = &[serde_json::Value::Null];
    let deserializer = SeqRefDeserializer::new(values);
    assert_eq!(deserializer.iter.as_slice(), values);
}

