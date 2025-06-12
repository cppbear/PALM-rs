// Answer 0

#[test]
fn test_as_slice() {
    // Arrange
    let buckets: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: 0, key: 1, value: 100 },
        Bucket { hash: 1, key: 2, value: 200 },
        Bucket { hash: 2, key: 3, value: 300 },
    ];
    let iter = Iter::new(&buckets);

    // Act
    let slice = iter.as_slice();

    // Assert
    assert_eq!(slice.entries.len(), buckets.len());
    assert_eq!(slice.entries[0].key, 1);
    assert_eq!(slice.entries[1].key, 2);
    assert_eq!(slice.entries[2].key, 3);
}

#[test]
fn test_as_slice_empty() {
    // Arrange
    let buckets: Vec<Bucket<i32, i32>> = Vec::new();
    let iter = Iter::new(&buckets);

    // Act
    let slice = iter.as_slice();

    // Assert
    assert_eq!(slice.entries.len(), 0);
}

