// Answer 0

#[test]
fn test_as_slice() {
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: "key1", value: "value1" },
        Bucket { hash: HashValue::default(), key: "key2", value: "value2" },
    ];

    let mut drain = Drain::new(buckets.clone().drain(..));
    let slice = drain.as_slice();

    assert_eq!(slice.entries.len(), 2);
    assert_eq!(slice.entries[0].key, "key1");
    assert_eq!(slice.entries[0].value, "value1");
    assert_eq!(slice.entries[1].key, "key2");
    assert_eq!(slice.entries[1].value, "value2");
}

#[test]
fn test_as_slice_empty() {
    let buckets: Vec<Bucket<&str, &str>> = Vec::new();
    let mut drain = Drain::new(buckets.clone().drain(..));
    let slice = drain.as_slice();

    assert_eq!(slice.entries.len(), 0);
}

