// Answer 0

#[test]
fn test_iter_mut_into_slice() {
    // Prepare test data
    let mut buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 1, key: 1, value: 10 },
        Bucket { hash: 2, key: 2, value: 20 },
        Bucket { hash: 3, key: 3, value: 30 },
    ];

    // Create an iterator
    let mut iter_mut = IterMut::new(&mut buckets);

    // Consume the iterator to create a mutable slice
    let slice = iter_mut.into_slice();

    // Verify the contents of the mutable slice
    assert_eq!(slice.entries.len(), 3);
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[2].key, 3);
}

