// Answer 0

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
    // other variants can be added as necessary
}

impl Value {
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }
}

#[test]
fn test_index_into_mut_non_array_variant() {
    let mut value = Value::Array(vec![]); // using an array variant
    let non_array_value = Value::from("not an array"); // should not match Value::Array
    let index = 0;

    let result = index.index_into_mut(&mut non_array_value);

    assert_eq!(result, None);
}

#[test]
fn test_index_into_mut_empty_array() {
    let mut value = Value::Array(vec![]); // empty array
    let index = 0;

    let result = index.index_into_mut(&mut value);

    assert_eq!(result, None);
}

#[test]
fn test_index_into_mut_with_values() {
    let mut value = Value::Array(vec![Value::from(1), Value::from(2), Value::from(3)]); // array with values
    let index = 5; // out of bounds index

    let result = index.index_into_mut(&mut value);

    assert_eq!(result, None);
}

