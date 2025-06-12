// Answer 0

#[test]
fn test_binary_search_by_empty_slice() {
    let slice: Slice<i32> = Slice::new();
    let result = slice.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_single_element_found() {
    let entries = vec![Bucket { hash: Default::default(), key: 3, value: () }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.binary_search_by(|&x| x.cmp(&3));
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_single_element_not_found() {
    let entries = vec![Bucket { hash: Default::default(), key: 3, value: () }];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_multiple_elements_found() {
    let entries = vec![
        Bucket { hash: Default::default(), key: 1, value: () },
        Bucket { hash: Default::default(), key: 2, value: () },
        Bucket { hash: Default::default(), key: 3, value: () },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.binary_search_by(|&x| x.cmp(&2));
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_multiple_elements_not_found() {
    let entries = vec![
        Bucket { hash: Default::default(), key: 1, value: () },
        Bucket { hash: Default::default(), key: 2, value: () },
        Bucket { hash: Default::default(), key: 3, value: () },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    let result = slice.binary_search_by(|&x| x.cmp(&4));
    assert_eq!(result, Err(3));
}

