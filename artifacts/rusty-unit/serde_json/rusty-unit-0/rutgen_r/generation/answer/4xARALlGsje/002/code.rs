// Answer 0

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
}

#[test]
fn test_index_into_with_empty_array() {
    let value = Value::Array(vec![]);
    let index: usize = 0;
    assert_eq!(index.index_into(&value), None);
}

#[test]
fn test_index_into_with_single_element_array() {
    let value = Value::Array(vec![Value::Array(vec![])]);
    let index: usize = 0;
    assert_eq!(index.index_into(&value), Some(&Value::Array(vec![])));
}

#[test]
fn test_index_into_with_multiple_elements_array() {
    let value = Value::Array(vec![
        Value::Array(vec![Value::Array(vec![])]),
        Value::Array(vec![Value::Array(vec![Value::Array(vec![Value::Array(vec![])])])]),
    ]);
    let index: usize = 1;
    assert_eq!(index.index_into(&value), Some(&Value::Array(vec![Value::Array(vec![Value::Array(vec![Value::Array(vec![])])])])));
}

#[test]
fn test_index_into_with_out_of_bounds_index() {
    let value = Value::Array(vec![Value::Array(vec![])]);
    let index: usize = 1;
    assert_eq!(index.index_into(&value), None);
}

