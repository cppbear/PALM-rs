// Answer 0

#[test]
fn test_as_slice_non_empty() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
        Bucket { hash: HashValue::default(), key: 2, value: "b" },
    ];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
}

#[test]
fn test_as_slice_empty() {
    let entries: Vec<Bucket<i32, &str>> = vec![];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

#[test]
#[should_panic]
fn test_as_slice_invalid_state() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "a" },
    ];
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    let invalid_slice = Slice::from_slice(&[]);
    assert_ne!(slice.entries.as_ptr(), invalid_slice.entries.as_ptr());
}

