// Answer 0

#[test]
fn test_binary_search_existing_value() {
    let sorted_set: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = sorted_set.binary_search(&3);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_missing_value() {
    let sorted_set: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = sorted_set.binary_search(&6);
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_empty() {
    let sorted_set: Vec<i32> = vec![];
    let result = sorted_set.binary_search(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_first_element() {
    let sorted_set: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = sorted_set.binary_search(&1);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_last_element() {
    let sorted_set: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result = sorted_set.binary_search(&5);
    assert_eq!(result, Ok(4));
}

