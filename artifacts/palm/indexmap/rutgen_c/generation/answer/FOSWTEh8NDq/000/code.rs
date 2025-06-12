// Answer 0

#[test]
fn test_iter_mut2_new() {
    // Arrange
    let mut bucket1 = Bucket { hash: 1, key: "key1", value: "value1" };
    let mut bucket2 = Bucket { hash: 2, key: "key2", value: "value2" };
    let mut buckets: Vec<Bucket<&str, &str>> = vec![bucket1, bucket2];

    // Act
    let iter_mut2 = IterMut2::new(&mut buckets);

    // Assert
    assert_eq!(iter_mut2.iter.len(), 2);
}

#[test]
fn test_iter_mut2_new_empty() {
    // Arrange
    let mut buckets: Vec<Bucket<&str, &str>> = Vec::new();

    // Act
    let iter_mut2 = IterMut2::new(&mut buckets);

    // Assert
    assert_eq!(iter_mut2.iter.len(), 0);
}

