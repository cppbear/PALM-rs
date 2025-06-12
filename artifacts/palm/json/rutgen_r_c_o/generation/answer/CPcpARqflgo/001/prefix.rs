// Answer 0

#[test]
fn test_index_or_insert_object() {
    let mut value = Value::Object(Map::new());
    let index = 0;
    index.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_null() {
    let mut value = Value::Null;
    let index = 0;
    index.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_bool() {
    let mut value = Value::Bool(true);
    let index = 0;
    index.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_number() {
    let mut value = Value::Number(Number);
    let index = 0;
    index.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_string() {
    let mut value = Value::String(String::from("test"));
    let index = 0;
    index.index_or_insert(&mut value);
}

#[test]
#[should_panic]
fn test_index_or_insert_empty_array() {
    let mut value = Value::Array(vec![]);
    let index = 0;
    index.index_or_insert(&mut value);
}

