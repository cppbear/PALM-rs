// Answer 0

#[test]
fn test_unexpected_null() {
    let value = Value::Null;
    let unexpected = value.unexpected();
    assert_eq!(unexpected, Unexpected::Unit);
}

#[test]
fn test_unexpected_bool() {
    let value_true = Value::Bool(true);
    let unexpected_true = value_true.unexpected();
    assert_eq!(unexpected_true, Unexpected::Bool(true));

    let value_false = Value::Bool(false);
    let unexpected_false = value_false.unexpected();
    assert_eq!(unexpected_false, Unexpected::Bool(false));
}

#[test]
fn test_unexpected_number() {
    struct MockNumber;
    impl Number for MockNumber {}

    let number = Number { n: MockNumber };
    let value = Value::Number(number);
    let unexpected = value.unexpected();
    #[cfg(feature = "arbitrary_precision")]
    assert_eq!(unexpected, Unexpected::Other("number"));
}

#[test]
fn test_unexpected_string() {
    let string_value = String::from("test");
    let value = Value::String(string_value);
    let unexpected = value.unexpected();
    assert_eq!(unexpected, Unexpected::Str("test"));
}

#[test]
fn test_unexpected_array() {
    let value = Value::Array(vec![]);
    let unexpected = value.unexpected();
    assert_eq!(unexpected, Unexpected::Seq);
}

#[test]
fn test_unexpected_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    let unexpected = value.unexpected();
    assert_eq!(unexpected, Unexpected::Map);
}

