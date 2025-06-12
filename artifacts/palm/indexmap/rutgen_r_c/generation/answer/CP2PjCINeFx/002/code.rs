// Answer 0

#[test]
fn test_truncate_no_op() {
    // Arrange
    let mut index_map = IndexMapCore::new();
    let len = 0; // as the default for a new IndexMapCore is 0
    // Act
    index_map.truncate(len);
    // Assert
    assert_eq!(index_map.len(), len);
}

#[test]
fn test_truncate_boundary_condition() {
    // Arrange
    let mut index_map = IndexMapCore::with_capacity(5);
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 2 });
    index_map.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 4 });
    let len = index_map.len(); // len should be 2
    // Act
    index_map.truncate(len);
    // Assert
    assert_eq!(index_map.len(), len);
}

