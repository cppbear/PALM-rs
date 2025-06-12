// Answer 0

#[test]
fn test_from_value_null() {
    let value = Value::Null;
    let result: Result<Option<()>, _> = from_value(value);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_from_value_bool() {
    let value = Value::Bool(true);
    let result: Result<bool, _> = from_value(value);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_from_value_number() {
    let value = Value::Number(Number::from_f64(12.5).unwrap());
    let result: Result<f64, _> = from_value(value);
    assert_eq!(result.unwrap(), 12.5);
}

#[test]
fn test_from_value_string() {
    let value = Value::String(String::from("test string"));
    let result: Result<String, _> = from_value(value);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
fn test_from_value_array() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let result: Result<Vec<bool>, _> = from_value(value);
    assert_eq!(result.unwrap(), vec![true, false]);
}

#[test]
fn test_from_value_object() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }

    let value = Value::Object(Map::from_iter(vec![
        (String::from("fingerprint"), Value::String(String::from("0xF9BA143B95FF6D82"))),
        (String::from("location"), Value::String(String::from("Menlo Park, CA"))),
    ]));

    let result: Result<User, _> = from_value(value);
    assert_eq!(
        result.unwrap(),
        User {
            fingerprint: String::from("0xF9BA143B95FF6D82"),
            location: String::from("Menlo Park, CA"),
        }
    );
}

#[test]
#[should_panic]
fn test_from_value_invalid_object() {
    let value = Value::Bool(true); // Not an object
    let _: Result<User, _> = from_value(value).unwrap();
}

