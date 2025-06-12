// Answer 0

#[test]
fn test_index_mut_with_empty_array() {
    let mut value = Value::Array(Vec::new());
    value[0] = Value::Bool(true);
}

#[test]
fn test_index_mut_with_non_empty_array() {
    let mut value = Value::Array(vec![Value::Bool(false), Value::Bool(false)]);
    value[1] = Value::Bool(true);
}

#[test]
#[should_panic]
fn test_index_mut_with_negative_index() {
    let mut value = Value::Array(Vec::new());
    value[-1] = Value::Bool(true);
}

#[test]
fn test_index_mut_with_insert_empty_object() {
    let mut value = Value::Null;
    value[""] = Value::Null;
}

#[test]
fn test_index_mut_with_insert_existing_key() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value["key"] = Value::Bool(false);
    value["key"] = Value::Bool(true);
}

#[test]
fn test_index_mut_with_insert_new_key() {
    let mut value = Value::Object(Map { map: MapImpl::new() });
    value["nonexistent"] = Value::Null;
}

#[test]
#[should_panic]
fn test_index_mut_with_non_object() {
    let mut value = Value::Bool(false);
    value["key"] = Value::Bool(true);
}

