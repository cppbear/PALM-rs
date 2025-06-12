// Answer 0

#[test]
fn test_index_into_valid_lower_bound() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index: usize = 0;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_valid_upper_bound() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index: usize = 1;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_empty_array() {
    let value = Value::Array(vec![]);
    let index: usize = 0;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_out_of_bounds() {
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index: usize = 2;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_array_of_objects() {
    let value = Value::Array(vec![
        Value::Object(Map::new()),
        Value::Object(Map::new()),
    ]);
    let index: usize = 1;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_array_of_numbers() {
    let value = Value::Array(vec![
        Value::Number(Number::from(1)),
        Value::Number(Number::from(2)),
    ]);
    let index: usize = 1;
    let _ = index.index_into(&value);
}

#[test]
fn test_index_into_array_of_strings() {
    let value = Value::Array(vec![
        Value::String(String::from("Hello")),
        Value::String(String::from("World")),
    ]);
    let index: usize = 0;
    let _ = index.index_into(&value);
}

