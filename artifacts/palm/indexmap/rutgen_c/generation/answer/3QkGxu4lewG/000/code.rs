// Answer 0

#[test]
fn test_slice_from_mut_slice() {
    // Arrange
    let mut buckets = [
        Bucket { hash: 1, key: 'a', value: 10 },
        Bucket { hash: 2, key: 'b', value: 20 },
        Bucket { hash: 3, key: 'c', value: 30 },
    ];

    // Act
    let slice = Slice::from_mut_slice(&mut buckets);

    // Assert
    assert_eq!(slice.entries.len(), 3);
    assert_eq!(slice.entries[0].key, 'a');
    assert_eq!(slice.entries[1].value, 20);
}

#[test]
fn test_slice_from_mut_slice_empty() {
    // Arrange
    let mut buckets: [Bucket<char, i32>; 0] = [];

    // Act
    let slice = Slice::from_mut_slice(&mut buckets);

    // Assert
    assert_eq!(slice.entries.len(), 0);
}

#[should_panic]
fn test_slice_from_mut_slice_invalid_pointer() {
    // Arrange
    let mut buckets: &mut [Bucket<char, i32>] = &mut [];
    let slice = Slice::from_mut_slice(buckets);
    
    // Act - trying to access the slice
    let _ = slice.entries[0]; // This should panic because the slice is empty
}

