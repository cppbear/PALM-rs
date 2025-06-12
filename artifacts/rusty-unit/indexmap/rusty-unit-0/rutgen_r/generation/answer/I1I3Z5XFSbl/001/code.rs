// Answer 0

#[test]
fn test_binary_search_existing_value() {
    let set: Vec<i32> = vec![1, 3, 5, 7, 9];
    let result = set.binary_search(&5);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_non_existing_value_insert_position() {
    let set: Vec<i32> = vec![1, 3, 5, 7, 9];
    let result = set.binary_search(&6);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_value_less_than_min() {
    let set: Vec<i32> = vec![1, 3, 5, 7, 9];
    let result = set.binary_search(&0);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_value_greater_than_max() {
    let set: Vec<i32> = vec![1, 3, 5, 7, 9];
    let result = set.binary_search(&10);
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_on_empty_set() {
    let set: Vec<i32> = vec![];
    let result = set.binary_search(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_repeated_values() {
    let set: Vec<i32> = vec![1, 1, 1, 1];
    let result = set.binary_search(&1);
    assert_eq!(result, Ok(0));
}

