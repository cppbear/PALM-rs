// Answer 0

#[test]
fn test_index_into_with_null() {
    let value = Value::Null;
    let index: usize = 0;
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_with_bool() {
    let value = Value::Bool(true);
    let index: usize = 0;
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_with_number() {
    let value = Value::Number(Number::from(12.5));
    let index: usize = 0;
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_with_string() {
    let value = Value::String(String::from("a string"));
    let index: usize = 0;
    let result = index.index_into(&value);
}

#[test]
fn test_index_into_with_object() {
    let value = Value::Object(Map::new());
    let index: usize = 0;
    let result = index.index_into(&value);
}

