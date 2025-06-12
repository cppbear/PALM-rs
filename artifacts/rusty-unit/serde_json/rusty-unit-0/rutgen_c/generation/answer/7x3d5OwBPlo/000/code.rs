// Answer 0

#[test]
fn test_as_number_with_integer() {
    let value = Value::Number(Number { n: 1 });
    assert_eq!(value.as_number(), Some(&Number { n: 1 }));
}

#[test]
fn test_as_number_with_float() {
    let value = Value::Number(Number { n: 2.2 });
    assert_eq!(value.as_number(), Some(&Number { n: 2.2 }));
}

#[test]
fn test_as_number_with_negative_integer() {
    let value = Value::Number(Number { n: -3 });
    assert_eq!(value.as_number(), Some(&Number { n: -3 }));
}

#[test]
fn test_as_number_with_string() {
    let value = Value::String(String::from("4"));
    assert_eq!(value.as_number(), None);
}

#[test]
fn test_as_number_with_null() {
    let value = Value::Null;
    assert_eq!(value.as_number(), None);
}

#[test]
fn test_as_number_with_bool() {
    let value = Value::Bool(true);
    assert_eq!(value.as_number(), None);
}

