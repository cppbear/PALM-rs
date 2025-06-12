// Answer 0

#[test]
fn test_index_into_mut_with_valid_index() {
    struct Indexer(usize);
    
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)]);
    let indexer = Indexer(1);
    
    assert_eq!(indexer.index_into_mut(&mut value), Some(&mut Value::Bool(false)));
}

#[test]
fn test_index_into_mut_with_out_of_bounds_index() {
    struct Indexer(usize);
    
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let indexer = Indexer(2);
    
    assert_eq!(indexer.index_into_mut(&mut value), None);
}

#[test]
fn test_index_into_mut_with_empty_array() {
    struct Indexer(usize);
    
    let mut value = Value::Array(vec![]);
    let indexer = Indexer(0);
    
    assert_eq!(indexer.index_into_mut(&mut value), None);
}

#[test]
fn test_index_into_mut_with_non_array_value() {
    struct Indexer(usize);
    
    let mut value = Value::Bool(true);
    let indexer = Indexer(0);
    
    assert_eq!(indexer.index_into_mut(&mut value), None);
}

