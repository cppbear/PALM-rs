// Answer 0

#[test]
fn test_binary_search_existing_value() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: () },
        Bucket { hash: HashValue::default(), key: 3, value: () },
        Bucket { hash: HashValue::default(), key: 5, value: () },
    ]});
    let result = slice.binary_search(&3);
    assert_eq!(result, Ok(1)); // The position of the value 3
}

#[test]
fn test_binary_search_missing_value() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: () },
        Bucket { hash: HashValue::default(), key: 3, value: () },
        Bucket { hash: HashValue::default(), key: 5, value: () },
    ]});
    let result = slice.binary_search(&4);
    assert_eq!(result, Err(2)); // The position where 4 can be inserted
}

#[test]
fn test_binary_search_first_value() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: () },
        Bucket { hash: HashValue::default(), key: 3, value: () },
        Bucket { hash: HashValue::default(), key: 5, value: () },
    ]});
    let result = slice.binary_search(&1);
    assert_eq!(result, Ok(0)); // The position of the value 1
}

#[test]
fn test_binary_search_last_value() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: () },
        Bucket { hash: HashValue::default(), key: 3, value: () },
        Bucket { hash: HashValue::default(), key: 5, value: () },
    ]});
    let result = slice.binary_search(&5);
    assert_eq!(result, Ok(2)); // The position of the value 5
}

#[test]
fn test_binary_search_not_in_empty_slice() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [] });
    let result = slice.binary_search(&42);
    assert_eq!(result, Err(0)); // Inserting into empty slice
}

#[test]
fn test_binary_search_negative_value() {
    let slice: Box<Slice<i32>> = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: -10, value: () },
        Bucket { hash: HashValue::default(), key: -5, value: () },
        Bucket { hash: HashValue::default(), key: 0, value: () },
    ]});
    let result = slice.binary_search(&-7);
    assert_eq!(result, Err(2)); // The position where -7 can be inserted
}

