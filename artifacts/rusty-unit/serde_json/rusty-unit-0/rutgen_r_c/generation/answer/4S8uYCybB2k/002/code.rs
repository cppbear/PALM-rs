// Answer 0

#[test]
fn test_as_u64_with_positive_integer() {
    let number_value = Value::Number(Number { n: N::PosInt(64) });
    assert_eq!(number_value.as_u64(), Some(64));
}

#[test]
fn test_as_u64_with_negative_integer() {
    let number_value = Value::Number(Number { n: N::NegInt(-64) });
    assert_eq!(number_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_floating_point() {
    let number_value = Value::Number(Number { n: N::Float(256.0) });
    assert_eq!(number_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_null_value() {
    let null_value = Value::Null;
    assert_eq!(null_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_bool_value() {
    let true_value = Value::Bool(true);
    assert_eq!(true_value.as_u64(), None);
    let false_value = Value::Bool(false);
    assert_eq!(false_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_string_value() {
    let string_value = Value::String(String::from("test"));
    assert_eq!(string_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_array_value() {
    let array_value = Value::Array(vec![Value::Number(Number { n: N::PosInt(1) })]);
    assert_eq!(array_value.as_u64(), None);
}

#[test]
fn test_as_u64_with_object_value() {
    let object_value = Value::Object(Map::new());
    assert_eq!(object_value.as_u64(), None);
}

