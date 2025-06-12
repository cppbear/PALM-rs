// Answer 0

#[test]
fn test_seq_ref_deserializer_with_null() {
    let slice: &[Value] = &[Value::Null];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_bool() {
    let slice: &[Value] = &[Value::Bool(true)];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_number() {
    let slice: &[Value] = &[Value::Number(Number::from(1))];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_string() {
    let slice: &[Value] = &[Value::String(String::from("test"))];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_array() {
    let slice: &[Value] = &[Value::Array(vec![Value::Bool(false)])];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_object() {
    let slice: &[Value] = &[Value::Object(Map::new())];
    SeqRefDeserializer::new(slice);
}

#[test]
fn test_seq_ref_deserializer_with_empty_slice() {
    let slice: &[Value] = &[];
    SeqRefDeserializer::new(slice);
}

