// Answer 0

#[test]
fn test_binary_search_found() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
        Bucket { hash: 2, key: 3, value: "c" },
    ]};
    let result = slice.binary_search(&2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_not_found() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
        Bucket { hash: 2, key: 3, value: "c" },
    ]};
    let result = slice.binary_search(&4);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_edge_case_empty() {
    let slice = Slice { entries: [] };
    let result = slice.binary_search(&1);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_first_element() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]};
    let result = slice.binary_search(&1);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_last_element() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]};
    let result = slice.binary_search(&2);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_less_than_first() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]};
    let result = slice.binary_search(&0);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_greater_than_last() {
    let slice = Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 1, key: 2, value: "b" },
    ]};
    let result = slice.binary_search(&3);
    assert_eq!(result, Err(2));
}

