// Answer 0

#[test]
fn test_display_null() {
    let value = Value::Null;
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

#[test]
fn test_display_bool() {
    let value = Value::Bool(true);
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

#[test]
fn test_display_number() {
    let value = Value::Number(Number::from(12.5));
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

#[test]
fn test_display_string() {
    let value = Value::String("a string".to_owned());
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

#[test]
fn test_display_array() {
    let value = Value::Array(vec![Value::String("an array".to_owned())]);
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

#[test]
fn test_display_object() {
    let mut map = Map::new();
    map.insert("key".to_owned(), Value::String("value".to_owned()));
    let value = Value::Object(map);
    let type_instance = Type(&value);
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    type_instance.fmt(formatter);
}

