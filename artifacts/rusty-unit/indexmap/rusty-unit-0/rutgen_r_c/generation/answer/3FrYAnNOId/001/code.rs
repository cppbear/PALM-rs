// Answer 0

#[test]
fn test_drain_new() {
    struct DummyKey;
    struct DummyValue;

    let buckets: Vec<Bucket<DummyKey, DummyValue>> = vec![
        Bucket { hash: HashValue::default(), key: DummyKey, value: DummyValue },
        Bucket { hash: HashValue::default(), key: DummyKey, value: DummyValue },
    ];
    
    let mut vec_buckets = buckets.clone();
    let drain = vec_buckets.drain(0..2);
    
    let result = Drain::new(drain);
    
    assert_eq!(result.iter.len(), 2);
}

#[test]
fn test_drain_empty() {
    struct DummyKey;
    struct DummyValue;

    let buckets: Vec<Bucket<DummyKey, DummyValue>> = vec![];
    let mut vec_buckets = buckets.clone();
    let drain = vec_buckets.drain(0..0);
    
    let result = Drain::new(drain);

    assert_eq!(result.iter.len(), 0);
}

