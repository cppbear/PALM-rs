// Answer 0

#[test]
fn test_new_with_empty_vec() {
    let input: Vec<Value> = vec![];
    let result = SeqDeserializer::new(input);
    assert_eq!(result.iter.len(), 0);
}

#[test]
fn test_new_with_single_value() {
    let input: Vec<Value> = vec![Value::String("single".to_string())];
    let result = SeqDeserializer::new(input);
    assert_eq!(result.iter.len(), 1);
}

#[test]
fn test_new_with_multiple_values() {
    let input: Vec<Value> = vec![Value::String("first".to_string()), Value::Number(42.into()), Value::Bool(true)];
    let result = SeqDeserializer::new(input);
    assert_eq!(result.iter.len(), 3);
}

#[should_panic]
fn test_new_with_large_vec() {
    let input: Vec<Value> = (0..100_000).map(|i| Value::Number(i.into())).collect();
    let _result = SeqDeserializer::new(input); // expect no panic, this is just a high limit test
}

