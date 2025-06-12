// Answer 0

#[test]
fn test_index_into_mut_valid_index() {
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index: usize = 0;
    let result = index.into_mut(&mut value);
}

#[test]
fn test_index_into_mut_boundary_index() {
    let mut value = Value::Array(vec![Value::Bool(true)]);
    let index: usize = 0;
    let result = index.into_mut(&mut value);
}

#[test]
fn test_index_into_mut_empty_array() {
    let mut value = Value::Array(vec![]);
    let index: usize = 0;
    let result = index.into_mut(&mut value);
}

#[test]
fn test_index_into_mut_exceeding_index() {
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index: usize = 2;
    let result = index.into_mut(&mut value);
}

