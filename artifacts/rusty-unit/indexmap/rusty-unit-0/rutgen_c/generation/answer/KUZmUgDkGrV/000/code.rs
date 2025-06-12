// Answer 0

#[test]
fn test_from_slice_valid() {
    let bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let buckets = [&bucket1, &bucket2];
    let slice = Slice::from_slice(&buckets);
    assert_eq!(slice.entries.len(), 2);
}

#[test]
fn test_from_slice_empty() {
    let buckets: &[Bucket<&str, &str>] = &[];
    let slice = Slice::from_slice(buckets);
    assert_eq!(slice.entries.len(), 0);
}

#[should_panic]
fn test_from_slice_invalid() {
    let buckets: &mut [Bucket<&str, &str>] = &mut [];
    let _slice = Slice::from_slice(buckets);
}

