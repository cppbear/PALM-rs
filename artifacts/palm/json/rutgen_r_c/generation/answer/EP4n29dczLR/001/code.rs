// Answer 0

#[test]
fn test_index_into_mut_none_for_non_array() {
    struct IndexImpl(usize);
    impl Index for IndexImpl {}

    let index = IndexImpl(0);
    
    let mut value_non_array = Value::Bool(true);
    let result = index.index_into_mut(&mut value_non_array);
    assert_eq!(result, None);
}

#[test]
fn test_index_into_mut_none_for_empty_array() {
    struct IndexImpl(usize);
    impl Index for IndexImpl {}

    let index = IndexImpl(0);
    
    let mut value_empty_array = Value::Array(vec![]);
    let result = index.index_into_mut(&mut value_empty_array);
    assert_eq!(result, None);
}

#[test]
fn test_index_into_mut_none_for_invalid_index() {
    struct IndexImpl(usize);
    impl Index for IndexImpl {}

    let index = IndexImpl(10);
    
    let mut value_with_array = Value::Array(vec![Value::String("valid".to_owned())]);
    let result = index.index_into_mut(&mut value_with_array);
    assert_eq!(result, None);
}

