// Answer 0

#[test]
fn test_iter_mut_new() {
    struct TestKey;
    struct TestValue;

    let mut buckets: Vec<Bucket<TestKey, TestValue>> = Vec::new();
    buckets.push(Bucket { hash: 0, key: TestKey, value: TestValue });
    buckets.push(Bucket { hash: 0, key: TestKey, value: TestValue });

    let mut iter_mut = IterMut::new(&mut buckets);
    assert_eq!(iter_mut.iter.as_slice().len(), 2);
}

#[test]
fn test_iter_mut_new_empty() {
    struct TestKey;
    struct TestValue;

    let mut buckets: Vec<Bucket<TestKey, TestValue>> = Vec::new();

    let mut iter_mut = IterMut::new(&mut buckets);
    assert_eq!(iter_mut.iter.as_slice().len(), 0);
}

