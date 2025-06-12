// Answer 0

#[test]
fn test_as_i64_not_number() {
    let v1 = Value::Null;
    let v2 = Value::Bool(true);
    let v3 = Value::String(String::from("not a number"));
    let v4 = Value::Array(vec![Value::Number(Number::from_f64(2.5).unwrap())]);
    let v5 = Value::Object(Map::<String, Value>::new());

    assert_eq!(v1.as_i64(), None);
    assert_eq!(v2.as_i64(), None);
    assert_eq!(v3.as_i64(), None);
    assert_eq!(v4.as_i64(), None);
    assert_eq!(v5.as_i64(), None);
}

