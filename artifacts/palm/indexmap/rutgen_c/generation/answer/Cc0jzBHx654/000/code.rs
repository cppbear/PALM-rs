// Answer 0

#[test]
fn test_as_slice() {
    let buckets = vec![
        Bucket { hash: 123, key: "key1", value: "value1" },
        Bucket { hash: 456, key: "key2", value: "value2" },
        Bucket { hash: 789, key: "key3", value: "value3" },
    ];
    
    let mut drain = Drain::new(buckets.clone().drain(..));
    let slice = drain.as_slice();
    
    assert_eq!(slice.entries.len(), buckets.len());
    for (i, bucket) in slice.entries.iter().enumerate() {
        assert_eq!(bucket.key, buckets[i].key);
        assert_eq!(bucket.value, buckets[i].value);
    }
}

#[test]
fn test_as_slice_empty() {
    let buckets: Vec<Bucket<&str, &str>> = Vec::new();
    let mut drain = Drain::new(buckets.clone().drain(..));
    let slice = drain.as_slice();
    
    assert_eq!(slice.entries.len(), 0);
}

