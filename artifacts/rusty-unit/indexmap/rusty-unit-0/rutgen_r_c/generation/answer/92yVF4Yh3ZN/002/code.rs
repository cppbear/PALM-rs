// Answer 0

#[test]
fn test_split_last_empty_slice() {
    let slice: Slice<i32> = Slice::new();
    assert_eq!(slice.split_last(), None);
}

#[test]
fn test_split_last_single_element_slice() {
    let bucket = Bucket { hash: 0, key: 1, value: 10 };
    let entries = [bucket];
    let slice = Slice::from_slice(&entries);
    assert_eq!(slice.split_last(), None);
}

#[test]
fn test_split_last_multiple_elements_slice() {
    let buckets = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 1, key: 2, value: 20 },
    ];
    let slice = Slice::from_slice(&buckets);
    assert_eq!(slice.split_last().map(|(key, _)| *key), Some(2));
}

