// Answer 0

#[test]
fn test_id_hasher_finish() {
    // Arrange
    let hasher_zero = IdHasher(0);
    let hasher_positive = IdHasher(42);
    let hasher_large = IdHasher(u64::MAX);
    
    // Act
    let result_zero = hasher_zero.finish();
    let result_positive = hasher_positive.finish();
    let result_large = hasher_large.finish();
    
    // Assert
    assert_eq!(result_zero, 0);
    assert_eq!(result_positive, 42);
    assert_eq!(result_large, u64::MAX);
}

