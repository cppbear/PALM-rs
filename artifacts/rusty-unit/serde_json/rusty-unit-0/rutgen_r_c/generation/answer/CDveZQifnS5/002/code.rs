// Answer 0

#[test]
fn test_size_hint_with_equal_lower_upper() {
    let values = vec![
        Value::Bool(true),
        Value::Number(Number::from(42)),
        Value::String("example".to_string()),
    ];
    
    let seq_deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    assert_eq!(seq_deserializer.size_hint(), Some(3));
}

#[test]
fn test_size_hint_with_one_element() {
    let values = vec![Value::Null];

    let seq_deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    assert_eq!(seq_deserializer.size_hint(), Some(1));
}

#[test]
fn test_size_hint_with_two_equal_elements() {
    let values = vec![Value::String("test".to_string()), Value::String("test".to_string())];

    let seq_deserializer = SeqDeserializer {
        iter: values.into_iter(),
    };

    assert_eq!(seq_deserializer.size_hint(), Some(2));
}

