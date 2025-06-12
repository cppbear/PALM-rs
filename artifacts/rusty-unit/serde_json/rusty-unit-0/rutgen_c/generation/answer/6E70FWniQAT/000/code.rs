// Answer 0

#[test]
fn test_new_with_empty_slice() {
    let slice: &[Value] = &[];
    let deserializer = SeqRefDeserializer::new(slice);
    assert_eq!(deserializer.iter.len(), 0);
}

#[test]
fn test_new_with_non_empty_slice() {
    let slice: &[Value] = &[
        Value::Null,
        Value::Bool(true),
        Value::Number(Number::from(12)),
        Value::String("test".to_string()),
        Value::Array(vec![Value::String("inner".to_string())]),
        Value::Object(Map::new()),
    ];
    let deserializer = SeqRefDeserializer::new(slice);
    assert_eq!(deserializer.iter.len(), 6);
}

#[test]
fn test_new_with_large_slice() {
    let slice: Vec<Value> = (0..1000).map(|i| Value::Number(Number::from(i))).collect();
    let deserializer = SeqRefDeserializer::new(&slice);
    assert_eq!(deserializer.iter.len(), 1000);
}

