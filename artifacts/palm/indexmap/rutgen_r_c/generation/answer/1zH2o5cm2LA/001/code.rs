// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    let entries = vec![
        Bucket { hash: HashValue::from(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue::from(2), key: "key2", value: "value2" },
    ];
    let iter = IntoIter::new(entries);
    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_new_with_empty_entries() {
    let entries: Vec<Bucket<&str, &str>> = Vec::new();
    let iter = IntoIter::new(entries);
    assert_eq!(iter.iter.len(), 0);
}

#[test]
fn test_new_with_single_entry() {
    let entries = vec![Bucket { hash: HashValue::from(3), key: "key3", value: "value3" }];
    let iter = IntoIter::new(entries);
    assert_eq!(iter.iter.len(), 1);
}

