// Answer 0

#[test]
fn test_index_into_mut_with_empty_array() {
    struct Indexer(usize);
    let mut value = Value::Array(vec![]);
    let indexer = Indexer(0);
    assert_eq!(indexer.index_into_mut(&mut value), None);
}

#[test]
fn test_index_into_mut_with_single_element_array() {
    struct Indexer(usize);
    let mut value = Value::Array(vec![Value::Bool(true)]);
    let indexer = Indexer(0);
    let result = indexer.index_into_mut(&mut value);
    assert!(result.is_some());
    if let Some(val) = result {
        match val {
            Value::Bool(b) => assert!(b),
            _ => panic!("Unexpected value type"),
        }
    }
}

#[test]
fn test_index_into_mut_with_nonexistent_index() {
    struct Indexer(usize);
    let mut value = Value::Array(vec![Value::Bool(true)]);
    let indexer = Indexer(1);
    assert_eq!(indexer.index_into_mut(&mut value), None);
}

#[test]
fn test_index_into_mut_with_multiple_elements() {
    struct Indexer(usize);
    let mut value = Value::Array(vec![Value::Bool(false), Value::Bool(true)]);
    let indexer = Indexer(1);
    let result = indexer.index_into_mut(&mut value);
    assert!(result.is_some());
    if let Some(val) = result {
        match val {
            Value::Bool(b) => assert!(b),
            _ => panic!("Unexpected value type"),
        }
    }
}

