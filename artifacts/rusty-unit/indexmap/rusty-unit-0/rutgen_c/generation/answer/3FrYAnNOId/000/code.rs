// Answer 0

#[test]
fn test_drain_new() {
    struct TestKey;
    struct TestValue;

    let bucket_vec: Vec<Bucket<TestKey, TestValue>> = vec![
        Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
        Bucket { hash: HashValue::default(), key: TestKey, value: TestValue },
    ];
    
    let drain_iter = bucket_vec.drain(..);
    let drain = Drain::new(drain_iter);

    assert_eq!(drain.iter.len(), 2);
}

#[test]
fn test_drain_new_empty() {
    struct TestKey;
    struct TestValue;

    let bucket_vec: Vec<Bucket<TestKey, TestValue>> = vec![];
    
    let drain_iter = bucket_vec.drain(..);
    let drain = Drain::new(drain_iter);

    assert_eq!(drain.iter.len(), 0);
}

