// Answer 0

#[test]
fn test_new_with_empty_vec() {
    let vec: Vec<Value> = Vec::new();
    let deserializer = SeqDeserializer::new(vec);
    assert!(deserializer.iter.len() == 0);
}

#[test]
fn test_new_with_single_value() {
    let vec = vec![Value::Number(Number::from(42))];
    let deserializer = SeqDeserializer::new(vec);
    assert!(deserializer.iter.len() == 1);
}

#[test]
fn test_new_with_multiple_values() {
    let vec = vec![
        Value::Bool(true),
        Value::String(String::from("test")),
        Value::Null,
    ];
    let deserializer = SeqDeserializer::new(vec);
    assert!(deserializer.iter.len() == 3);
}

#[test]
fn test_new_with_nested_array() {
    let vec = vec![
        Value::Array(vec![Value::Number(Number::from(1))]),
        Value::Array(vec![Value::Bool(false)]),
    ];
    let deserializer = SeqDeserializer::new(vec);
    assert!(deserializer.iter.len() == 2);
}

