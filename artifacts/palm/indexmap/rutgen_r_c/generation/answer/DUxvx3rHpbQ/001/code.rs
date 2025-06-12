// Answer 0

#[test]
fn test_binary_search_by_empty_slice() {
    let slice = Box::new(Slice { entries: [] });
    let result = slice.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_single_element_found() {
    let slice = Box::new(Slice { entries: [Bucket { hash: 0, key: 5, value: () }] });
    let result = slice.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_single_element_not_found() {
    let slice = Box::new(Slice { entries: [Bucket { hash: 0, key: 5, value: () }] });
    let result = slice.binary_search_by(|&x| x.cmp(&3));
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_multiple_elements_found() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: () },
        Bucket { hash: 0, key: 3, value: () },
        Bucket { hash: 0, key: 5, value: () },
        Bucket { hash: 0, key: 7, value: () },
    ] });
    let result = slice.binary_search_by(|&x| x.cmp(&5));
    assert_eq!(result, Ok(2));
}

#[test]
fn test_binary_search_by_multiple_elements_not_found() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: () },
        Bucket { hash: 0, key: 3, value: () },
        Bucket { hash: 0, key: 5, value: () },
        Bucket { hash: 0, key: 7, value: () },
    ] });
    let result = slice.binary_search_by(|&x| x.cmp(&4));
    assert_eq!(result, Err(2));
}

#[test]
fn test_binary_search_by_multiple_elements_first() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: () },
        Bucket { hash: 0, key: 3, value: () },
        Bucket { hash: 0, key: 5, value: () },
        Bucket { hash: 0, key: 7, value: () },
    ] });
    let result = slice.binary_search_by(|&x| x.cmp(&1));
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_multiple_elements_last() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: () },
        Bucket { hash: 0, key: 3, value: () },
        Bucket { hash: 0, key: 5, value: () },
        Bucket { hash: 0, key: 7, value: () },
    ] });
    let result = slice.binary_search_by(|&x| x.cmp(&7));
    assert_eq!(result, Ok(3));
}

