// Answer 0

#[test]
fn test_binary_search_by_key_empty() {
    let slice: Box<Slice<i32>> = Box::new(Slice::new());
    let key = &1;
    let result = slice.binary_search_by_key(key, |x| *x);
    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_key_not_found() {
    let entries = vec![
        Bucket { hash: 1, key: 1, value: "one" },
        Bucket { hash: 2, key: 2, value: "two" },
        Bucket { hash: 3, key: 3, value: "three" },
    ];
    let slice = Box::new(Slice { entries });
    let key = &4;
    let result = slice.binary_search_by_key(key, |x| *x);
    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_key_found() {
    let entries = vec![
        Bucket { hash: 1, key: 1, value: "one" },
        Bucket { hash: 2, key: 2, value: "two" },
        Bucket { hash: 3, key: 3, value: "three" },
    ];
    let slice = Box::new(Slice { entries });
    let key = &2;
    let result = slice.binary_search_by_key(key, |x| *x);
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_key_first_element() {
    let entries = vec![
        Bucket { hash: 1, key: 1, value: "one" },
        Bucket { hash: 2, key: 2, value: "two" },
    ];
    let slice = Box::new(Slice { entries });
    let key = &1;
    let result = slice.binary_search_by_key(key, |x| *x);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_binary_search_by_key_last_element() {
    let entries = vec![
        Bucket { hash: 1, key: 1, value: "one" },
        Bucket { hash: 2, key: 3, value: "three" },
    ];
    let slice = Box::new(Slice { entries });
    let key = &3;
    let result = slice.binary_search_by_key(key, |x| *x);
    assert_eq!(result, Ok(1));
}

