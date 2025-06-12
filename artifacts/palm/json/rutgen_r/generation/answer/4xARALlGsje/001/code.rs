// Answer 0

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
    Other,
}

impl Value {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
}

#[test]
fn test_index_into_non_array() {
    let value = Value::Other;
    let index = 0; // arbitrary index

    let result = index.index_into(&value);
    assert_eq!(result, None);
}

#[test]
fn test_index_into_empty_array() {
    let value = Value::Array(vec![]);
    let index = 0; // arbitrary index

    let result = index.index_into(&value);
    assert_eq!(result, None);
}

#[test]
fn test_index_into_non_empty_array() {
    let value = Value::Array(vec![Value::Other, Value::Other]);
    let index = 2; // out of bounds index

    let result = index.index_into(&value);
    assert_eq!(result, None);
}

