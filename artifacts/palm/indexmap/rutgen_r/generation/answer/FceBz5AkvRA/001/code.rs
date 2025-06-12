// Answer 0

#[test]
fn test_binary_search_by_found() {
    let set = vec![1, 3, 5, 7, 9];  // Sorted input
    let result = set.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Ok(2));  // Value found at index 2
}

#[test]
fn test_binary_search_by_not_found() {
    let set = vec![1, 3, 5, 7, 9];  // Sorted input
    let result = set.binary_search_by(|&x| x.cmp(&4));
    assert_eq!(result, Err(2));  // Should return the insertion point index
}

#[test]
fn test_binary_search_by_insert_at_beginning() {
    let set = vec![2, 4, 6, 8, 10];  // Sorted input
    let result = set.binary_search_by(|&x| x.cmp(&1));
    assert_eq!(result, Err(0));  // Insert before the first element
}

#[test]
fn test_binary_search_by_insert_at_end() {
    let set = vec![2, 4, 6, 8, 10];  // Sorted input
    let result = set.binary_search_by(|&x| x.cmp(&12));
    assert_eq!(result, Err(5));  // Insert after the last element
}

#[test]
fn test_binary_search_by_empty_set() {
    let set: Vec<i32> = Vec::new();  // Empty input
    let result = set.binary_search_by(|&x| x.cmp(&1));
    assert_eq!(result, Err(0));  // Insert at the beginning of an empty set
}

