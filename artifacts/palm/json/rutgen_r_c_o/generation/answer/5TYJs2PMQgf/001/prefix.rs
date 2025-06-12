// Answer 0

#[test]
fn test_invalid_type_null() {
    let value = Value::Null;
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_bool_true() {
    let value = Value::Bool(true);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_bool_false() {
    let value = Value::Bool(false);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_number_integer() {
    let number = Number { n: 0 };
    let value = Value::Number(number);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_number_float() {
    let number = Number { n: 3.14 };
    let value = Value::Number(number);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_string_empty() {
    let value = Value::String(String::from(""));
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_string_non_empty() {
    let value = Value::String(String::from("a string"));
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_array_empty() {
    let value = Value::Array(vec![]);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_array_non_empty() {
    let value = Value::Array(vec![Value::String(String::from("item"))]);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_object_empty() {
    let value = Value::Object(Map::new());
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_object_non_empty() {
    let mut map = Map::new();
    map.insert(String::from("key"), Value::String(String::from("value")));
    let value = Value::Object(map);
    let exp: &dyn Expected = &();
    value.invalid_type(exp);
}

