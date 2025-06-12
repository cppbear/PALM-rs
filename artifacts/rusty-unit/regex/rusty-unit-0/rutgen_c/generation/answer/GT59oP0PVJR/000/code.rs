// Answer 0

#[test]
fn test_clear_increments_version() {
    // Arrange
    let mut cache = SuffixCache::new(5);
    let initial_version = cache.version;

    // Act
    cache.clear();

    // Assert
    assert_eq!(cache.version, initial_version + 1);
}

