// Answer 0

#[test]
fn test_binary_search_found() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&5);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_not_found() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&4);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_first_element() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&1);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_last_element() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&9);
    assert_eq!(result, Ok(4));
}

#[test]
fn test_binary_search_not_present_high() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&10);
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_not_present_low() {
    let sorted_set = vec![1, 3, 5, 7, 9];
    let result = sorted_set.binary_search(&0);
    assert_eq!(result, Err(0));
}

