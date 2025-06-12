// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    let values = vec![Value::Null, Value::Bool(true), Value::Number(Number::from(10))];
    let iter = values.clone().into_iter();
    let seq_deserializer = SeqDeserializer { iter };

    assert_eq!(seq_deserializer.size_hint(), Some(3));
}

#[test]
fn test_size_hint_lower_not_equal_upper() {
    let values = vec![Value::Null, Value::Bool(true)];
    let iter = values.into_iter();
    let seq_deserializer = SeqDeserializer { iter };

    assert_eq!(seq_deserializer.size_hint(), None);
}

#[test]
fn test_size_hint_empty_iter() {
    let values: Vec<Value> = Vec::new();
    let iter = values.into_iter();
    let seq_deserializer = SeqDeserializer { iter };

    assert_eq!(seq_deserializer.size_hint(), Some(0));
}

#[test]
fn test_size_hint_iter_with_single_element() {
    let values = vec![Value::String("single".to_owned())];
    let iter = values.into_iter();
    let seq_deserializer = SeqDeserializer { iter };

    assert_eq!(seq_deserializer.size_hint(), Some(1));
}

