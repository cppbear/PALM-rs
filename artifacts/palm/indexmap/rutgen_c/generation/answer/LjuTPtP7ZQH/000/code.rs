// Answer 0

#[test]
fn test_from_boxed() {
    // Setup a sample Bucket array
    let buckets: Box<[Bucket<i32, i32>]> = Box::new([
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ]);
    
    // Call the from_boxed function
    let slice = Slice::from_boxed(buckets);

    // Verify the contents of the resulting Slice
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[0].value, 10);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[1].value, 20);
}

#[test]
fn test_from_boxed_empty() {
    // Setup an empty Bucket array
    let buckets: Box<[Bucket<i32, i32>]> = Box::new([]);
    
    // Call the from_boxed function
    let slice = Slice::from_boxed(buckets);

    // Verify the contents of the resulting Slice
    assert!(slice.entries.is_empty());
}

