// Answer 0

#[test]
fn test_seq_deserializer_new_with_empty_vector() {
    let vec: Vec<Value> = Vec::new();
    let deserializer = SeqDeserializer::new(vec);
    assert_eq!(deserializer.iter.count(), 0);
}

#[test]
fn test_seq_deserializer_new_with_non_empty_vector() {
    let vec = vec![
        Value::Null,
        Value::Bool(true),
        Value::Number(Number::from(42)),
        Value::String(String::from("test")),
    ];
    let deserializer = SeqDeserializer::new(vec);
    assert_eq!(deserializer.iter.count(), 4);
}

#[test]
fn test_seq_deserializer_new_with_array() {
    let vec = vec![Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))])];
    let deserializer = SeqDeserializer::new(vec);
    assert_eq!(deserializer.iter.count(), 1);
}

