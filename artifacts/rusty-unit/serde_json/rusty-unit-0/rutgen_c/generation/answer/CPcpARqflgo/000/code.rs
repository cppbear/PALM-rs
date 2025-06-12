// Answer 0

#[test]
fn test_index_or_insert_array_existing_index() {
    struct ArrayIndex(usize);
    
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = ArrayIndex(1);
    
    let result = index.index_or_insert(&mut value);
    match result {
        Value::Bool(false) => {}
        _ => panic!("Expected Value::Bool(false)"),
    }
}

#[test]
#[should_panic(expected = "cannot access index 2 of JSON array of length 2")]
fn test_index_or_insert_array_out_of_bounds() {
    struct ArrayIndex(usize);
    
    let mut value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = ArrayIndex(2);
    
    let _ = index.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_single_element_array() {
    struct ArrayIndex(usize);
    
    let mut value = Value::Array(vec![Value::Bool(true)]);
    let index = ArrayIndex(0);
    
    let result = index.index_or_insert(&mut value);
    match result {
        Value::Bool(true) => {}
        _ => panic!("Expected Value::Bool(true)"),
    }
} 

#[test]
#[should_panic(expected = "cannot access index 0 of JSON array of length 0")]
fn test_index_or_insert_empty_array() {
    struct ArrayIndex(usize);
    
    let mut value = Value::Array(vec![]);
    let index = ArrayIndex(0);
    
    let _ = index.index_or_insert(&mut value);
}

