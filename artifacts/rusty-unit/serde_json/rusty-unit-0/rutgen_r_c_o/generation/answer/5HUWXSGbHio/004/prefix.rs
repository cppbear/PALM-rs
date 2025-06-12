// Answer 0

#[test]
fn test_fmt_null() {
    let value = Value::Null;
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_bool() {
    let value = Value::Bool(true);
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_number_min() {
    let value = Value::Number(Number::from(i64::MIN));
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_number_max() {
    let value = Value::Number(Number::from(i64::MAX));
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_string() {
    let value = Value::String(String::from("test"));
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_array() {
    let value = Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]);
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

#[test]
fn test_fmt_object() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let type_value = Type(&value);
    let mut formatter = fmt::Formatter::default();
    let _ = type_value.fmt(&mut formatter);
}

