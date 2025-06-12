// Answer 0

fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
    match v {
        Value::Array(vec) => vec.get_mut(*self),
        _ => None,
    }
}

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
}

#[test]
fn test_index_into_mut_valid_index() {
    let mut v = Value::Array(vec![Value::Array(vec![]), Value::Array(vec![])]);
    let index = 1;
    let result = index_into_mut(&index, &mut v);
    assert!(result.is_some());
}

#[test]
fn test_index_into_mut_invalid_index() {
    let mut v = Value::Array(vec![Value::Array(vec![])]);
    let index = 1;
    let result = index_into_mut(&index, &mut v);
    assert!(result.is_none());
}

#[test]
fn test_index_into_mut_empty_array() {
    let mut v = Value::Array(vec![]);
    let index = 0;
    let result = index_into_mut(&index, &mut v);
    assert!(result.is_none());
}

#[test]
fn test_index_into_mut_array_with_nested_arrays() {
    let mut v = Value::Array(vec![Value::Array(vec![Value::Array(vec![])]), Value::Array(vec![Value::Array(vec![Value::Array(vec![])])])]);
    let index = 0;
    let result = index_into_mut(&index, &mut v);
    assert!(result.is_some());
}

