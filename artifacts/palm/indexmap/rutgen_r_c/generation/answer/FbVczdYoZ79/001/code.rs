// Answer 0

#[test]
fn test_binary_search_by_found_element() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };
    
    let result = slice.binary_search_by(|key, _| {
        if *key < 2 {
            Ordering::Less
        } else if *key > 2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found_before() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by(|key, _| {
        if *key < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(0));
}

#[test]
fn test_binary_search_by_not_found_after() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by(|key, _| {
        if *key < 4 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(3));
}

#[test]
fn test_binary_search_by_insert_position() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 3, value: "c" },
    ];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by(|key, _| {
        if *key < 2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_empty_slice() {
    let entries: Vec<Bucket<i32, &str>> = vec![];
    let slice = Slice { entries: entries.try_into().unwrap() };

    let result = slice.binary_search_by(|key, _| {
        if *key < 1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    assert_eq!(result, Err(0));
}

