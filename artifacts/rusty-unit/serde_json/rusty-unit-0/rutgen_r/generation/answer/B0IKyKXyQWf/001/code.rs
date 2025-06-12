// Answer 0

#[derive(Debug)]
struct Value {
    data: Vec<i32>,
}

impl Value {
    fn new(data: Vec<i32>) -> Self {
        Value { data }
    }
    
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if self.data.is_empty() {
            panic!("Cannot index into an empty Value");
        }
        v
    }
}

#[test]
fn test_index_or_insert_empty() {
    let mut new_value = Value::new(vec![1, 2, 3]);
    let empty_value = Value::new(vec![]);
    
    // This test should panic because we are trying to index into an empty Value
    let result = std::panic::catch_unwind(|| {
        empty_value.index_or_insert(&mut new_value);
    });
    
    assert!(result.is_err());
}

#[test]
fn test_index_or_insert_non_empty() {
    let mut new_value = Value::new(vec![4, 5, 6]);
    let non_empty_value = Value::new(vec![1, 2, 3]);
    
    // This should return a mutable reference to new_value
    let result = non_empty_value.index_or_insert(&mut new_value);
    
    assert_eq!(result.data, vec![4, 5, 6]);
}

