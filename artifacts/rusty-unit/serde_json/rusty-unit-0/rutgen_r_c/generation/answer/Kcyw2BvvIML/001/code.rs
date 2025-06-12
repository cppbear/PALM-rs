// Answer 0

#[test]
fn test_to_value_null() {
    let v: Result<Value, Error> = to_value(None::<()>);
    assert!(v.is_ok());
    assert_eq!(v.unwrap(), Value::Null);
}

#[test]
fn test_to_value_bool() {
    let v_true: Result<Value, Error> = to_value(true);
    let v_false: Result<Value, Error> = to_value(false);
    assert_eq!(v_true.unwrap(), Value::Bool(true));
    assert_eq!(v_false.unwrap(), Value::Bool(false));
}

#[test]
fn test_to_value_number() {
    let v_int: Result<Value, Error> = to_value(42);
    let v_float: Result<Value, Error> = to_value(3.14);
    assert!(v_int.is_ok());
    assert!(v_float.is_ok());
}

#[test]
fn test_to_value_string() {
    let v: Result<Value, Error> = to_value("Hello, world!");
    assert!(v.is_ok());
    assert_eq!(v.unwrap(), Value::String("Hello, world!".to_owned()));
}

#[test]
fn test_to_value_array() {
    let v: Result<Value, Error> = to_value(vec![1, 2, 3]);
    assert!(v.is_ok());
    assert_eq!(
        v.unwrap(),
        Value::Array(vec![
            Value::Number(Number::from(1)),
            Value::Number(Number::from(2)),
            Value::Number(Number::from(3))
        ])
    );
}

#[test]
fn test_to_value_object() {
    use serde::Serialize;
    #[derive(Serialize)]
    struct User {
        fingerprint: String,
        location: String,
    }
    let user = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };
    
    let expected = Value::Object(Map::new().insert("fingerprint".to_owned(), Value::String("0xF9BA143B95FF6D82".to_owned())).insert("location".to_owned(), Value::String("Menlo Park, CA".to_owned())));
    let v: Result<Value, Error> = to_value(user);
    assert!(v.is_ok());
    assert_eq!(v.unwrap(), expected);
}

#[test]
#[should_panic]
fn test_to_value_invalid_map_keys() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");
    let _: Result<Value, Error> = to_value(map);
}

