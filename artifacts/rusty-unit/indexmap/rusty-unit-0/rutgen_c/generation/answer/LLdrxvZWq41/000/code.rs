// Answer 0

#[test]
fn test_as_slice() {
    let mut buckets = [
        Bucket { hash: 1, key: 1, value: "a" },
        Bucket { hash: 2, key: 2, value: "b" },
        Bucket { hash: 3, key: 3, value: "c" },
    ];

    let iter_mut = IterMut::new(&mut buckets);
    let slice = iter_mut.as_slice();
    
    assert_eq!(slice.entries.len(), buckets.len());
    
    for (i, bucket) in slice.entries.iter().enumerate() {
        assert_eq!(bucket.key, buckets[i].key);
        assert_eq!(bucket.value, buckets[i].value);
    }
}

#[test]
fn test_as_slice_empty() {
    let mut buckets: [Bucket<i32, &str>; 0] = [];

    let iter_mut = IterMut::new(&mut buckets);
    let slice = iter_mut.as_slice();
    
    assert_eq!(slice.entries.len(), buckets.len());
}

