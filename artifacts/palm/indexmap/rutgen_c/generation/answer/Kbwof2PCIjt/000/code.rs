// Answer 0

#[test]
fn test_as_slice_empty() {
    let entries: Vec<Bucket<i32, &str>> = Vec::new();
    let iter = IntoIter::new(entries);
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

#[test]
fn test_as_slice_non_empty() {
    let entries = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "one" },
        Bucket { hash: HashValue::default(), key: 2, value: "two" },
    ];
    let iter = IntoIter::new(entries.clone());
    let slice = iter.as_slice();
    assert_eq!(slice.entries.len(), entries.len());
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, "one");
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[1].value, "two");
}

