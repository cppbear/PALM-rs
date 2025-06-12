// Answer 0

#[derive(Debug)]
struct Value {
    data: Vec<Value>,
}

impl Value {
    fn array(data: Vec<Value>) -> Self {
        Value { data }
    }
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
fn test_index_into_valid() {
    let value_array = Value::array(vec![
        Value::array(vec![]), 
        Value::array(vec![]), 
        Value::array(vec![]) 
    ]);
    
    let index = Value::array(vec![]);
    let result = index.index_into(&value_array);
    assert_eq!(result, Some(&value_array.data[0]));
}

#[test]
fn test_index_into_out_of_bounds() {
    let value_array = Value::array(vec![
        Value::array(vec![]), 
        Value::array(vec![]) 
    ]);
    
    let index = Value::array(vec![2]); // Indexing with a value larger than the array size
    let result = index.index_into(&value_array);
    assert_eq!(result, None);
}

#[test]
fn test_index_into_non_array() {
    let non_array_value = Value::array(vec![]);
    
    let index = Value::array(vec![]);
    let result = index.index_into(&non_array_value);
    assert_eq!(result, None);
}

