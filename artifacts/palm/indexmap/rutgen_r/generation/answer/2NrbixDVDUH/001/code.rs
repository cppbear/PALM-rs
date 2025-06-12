// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    let set = vec![1, 3, 5, 7, 9];
    let value = 5;
    let result = binary_search_by_key(&set, &value, |&x| x);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_key_not_found() {
    let set = vec![1, 3, 5, 7, 9];
    let value = 4;
    let result = binary_search_by_key(&set, &value, |&x| x);
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_by_key_insert_position() {
    let set = vec![1, 3, 5, 7, 9];
    let value = 0;
    let result = binary_search_by_key(&set, &value, |&x| x);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_last_position() {
    let set = vec![1, 3, 5, 7, 9];
    let value = 10;
    let result = binary_search_by_key(&set, &value, |&x| x);
    assert_eq!(result, Err(5));
}

#[test]
fn test_binary_search_by_key_empty_set() {
    let set: Vec<i32> = vec![];
    let value = 1;
    let result = binary_search_by_key(&set, &value, |&x| x);
    assert_eq!(result, Err(0));
}

