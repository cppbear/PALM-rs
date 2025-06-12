// Answer 0

#[test]
fn test_drain_new() {
    // Arrange
    let buckets = vec![
        Bucket { hash: HashValue::default(), key: 1, value: "value1" },
        Bucket { hash: HashValue::default(), key: 2, value: "value2" },
    ];
    let mut vec_buckets = buckets.clone();
    let drain = vec_buckets.drain(..); // Create a drain

    // Act
    let drain_instance: Drain<i32, &str> = Drain::new(drain);

    // Assert
    assert_eq!(drain_instance.iter.len(), buckets.len());
}

#[test]
fn test_drain_empty() {
    // Arrange
    let buckets: Vec<Bucket<i32, &str>> = Vec::new();
    let mut vec_buckets = buckets.clone();
    let drain = vec_buckets.drain(..); // Create a drain

    // Act
    let drain_instance: Drain<i32, &str> = Drain::new(drain);

    // Assert
    assert_eq!(drain_instance.iter.len(), 0);
}

