// Answer 0

#[test]
fn test_index_into_mut_null() {
    let index: usize = 0;
    let mut value = Value::Null;
    index.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_bool() {
    let index: usize = 0;
    let mut value = Value::Bool(true);
    index.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_number() {
    let index: usize = 0;
    let mut value = Value::Number(Number);
    index.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_string() {
    let index: usize = 0;
    let mut value = Value::String(String::from("a string"));
    index.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_object() {
    let index: usize = 10;
    let mut value = Value::Object(Map::new());
    index.index_into_mut(&mut value);
}

