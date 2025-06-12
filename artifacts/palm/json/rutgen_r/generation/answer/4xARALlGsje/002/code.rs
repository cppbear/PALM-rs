// Answer 0

#[derive(Debug)]
struct TestIndex(usize);

impl TestIndex {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(self.0),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Value {
    Array(Vec<Value>),
}

#[test]
fn test_index_into_valid() {
    let array_value = Value::Array(vec![
        Value::Array(vec![]), // Index 0
        Value::Array(vec![]), // Index 1
        Value::Array(vec![]), // Index 2
    ]);
    
    let index = TestIndex(1);
    assert!(index.index_into(&array_value).is_some());
}

#[test]
fn test_index_into_out_of_bounds() {
    let array_value = Value::Array(vec![
        Value::Array(vec![]), // Index 0
        Value::Array(vec![]), // Index 1
    ]);
    
    let index = TestIndex(2);
    assert!(index.index_into(&array_value).is_none());
}

#[test]
fn test_index_into_empty_array() {
    let array_value = Value::Array(vec![]);
    
    let index = TestIndex(0);
    assert!(index.index_into(&array_value).is_none());
}

#[test]
fn test_index_into_non_array_value() {
    let non_array_value = Value::Array(vec![]);
    
    let index = TestIndex(0);
    assert!(index.index_into(&non_array_value).is_none());
}

