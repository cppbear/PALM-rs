// Answer 0

#[test]
fn test_index_or_insert_with_valid_index() {
    let mut value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let index: usize = 1;
    let result = index_or_insert(&index, &mut value);
}

#[test]
#[should_panic(expected = "cannot access index 10 of JSON array of length 2")]
fn test_index_or_insert_with_index_out_of_bounds() {
    let mut value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let index: usize = 10;
    let result = index_or_insert(&index, &mut value);
}

#[test]
#[should_panic(expected = "cannot access index 0 of JSON array of length 0")]
fn test_index_or_insert_with_empty_array() {
    let mut value = Value::Array(vec![]);
    let index: usize = 0;
    let result = index_or_insert(&index, &mut value);
}

#[test]
fn test_index_or_insert_with_last_index() {
    let mut value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let index: usize = 1;
    let result = index_or_insert(&index, &mut value);
}

#[test]
fn test_index_or_insert_with_zero_index() {
    let mut value = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let index: usize = 0;
    let result = index_or_insert(&index, &mut value);
}

