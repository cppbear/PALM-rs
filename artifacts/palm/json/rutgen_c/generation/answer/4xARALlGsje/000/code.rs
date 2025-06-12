// Answer 0

#[test]
fn test_index_into_array() {
    struct IndexImpl(usize);
    
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = IndexImpl(0);
    
    assert_eq!(index.index_into(&value), Some(&Value::Bool(true)));
}

#[test]
fn test_index_into_array_out_of_bounds() {
    struct IndexImpl(usize);
    
    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = IndexImpl(2);
    
    assert_eq!(index.index_into(&value), None);
}

#[test]
fn test_index_into_non_array() {
    struct IndexImpl(usize);
    
    let value = Value::Bool(true);
    let index = IndexImpl(0);
    
    assert_eq!(index.index_into(&value), None);
}

#[test]
fn test_index_into_empty_array() {
    struct IndexImpl(usize);
    
    let value = Value::Array(vec![]);
    let index = IndexImpl(0);
    
    assert_eq!(index.index_into(&value), None);
}

