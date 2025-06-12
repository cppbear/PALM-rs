// Answer 0

#[test]
fn test_shift_remove_full_one_element_found() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);
    
    let mut index_map = IndexMap::<KeyType, ValueType, RandomState>::new();
    index_map.insert(KeyType(1), ValueType(100));
    
    let result = index_map.shift_remove_full(&KeyType(1));
    assert_eq!(result, Some((0, KeyType(1), ValueType(100))));
}

#[test]
fn test_shift_remove_full_one_element_not_found() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);
    
    let mut index_map = IndexMap::<KeyType, ValueType, RandomState>::new();
    index_map.insert(KeyType(1), ValueType(100));
    
    let result = index_map.shift_remove_full(&KeyType(2));
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_full_multiple_elements_found() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);
    
    let mut index_map = IndexMap::<KeyType, ValueType, RandomState>::new();
    index_map.insert(KeyType(1), ValueType(100));
    index_map.insert(KeyType(2), ValueType(200));
    index_map.insert(KeyType(3), ValueType(300));
    
    let result = index_map.shift_remove_full(&KeyType(2));
    assert_eq!(result, Some((1, KeyType(2), ValueType(200))));
}

#[test]
fn test_shift_remove_full_empty_map() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);
    
    let mut index_map = IndexMap::<KeyType, ValueType, RandomState>::new();
    
    let result = index_map.shift_remove_full(&KeyType(1));
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_full_one_element_key_equivalent_panic() {
    use std::panic;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct KeyType(i32);
    
    #[derive(Debug)]
    struct ValueType(i32);
    
    let mut index_map = IndexMap::<KeyType, ValueType, RandomState>::new();
    index_map.insert(KeyType(1), ValueType(100));
    
    let panic_result = panic::catch_unwind(|| {
        index_map.shift_remove_full(&KeyType(1));
    });
    
    assert!(panic_result.is_ok());
}

