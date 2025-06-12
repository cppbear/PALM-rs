// Answer 0

#[test]
fn test_split_to_must_use() {
    use bytes::BytesMut;

    // Test: Should not panic and must use the result
    let mut b1 = BytesMut::from("hello world");
    let result1 = b1.split_to(6);
    assert_eq!(result1.len(), 6);  // Ensure result length is as expected
    assert_eq!(b1.len(), 5);       // Verify remaining length in original buffer

    // Test: Attempt to split with zero length
    let mut b2 = BytesMut::from("hello!");
    let result2 = b2.split_to(0);
    assert_eq!(result2.len(), 0);  // Result should be empty
    assert_eq!(b2.len(), 6);       // Original buffer should remain unchanged

    // Test: Attempt to split exactly to the length of buffer
    let mut b3 = BytesMut::from("Rust is great");
    let result3 = b3.split_to(13);
    assert_eq!(result3.len(), 13); // Result should match the original length
    assert!(b3.is_empty());        // Original buffer should be empty

    // Test: Panic condition when splitting more than available length
    let mut b4 = BytesMut::from("too short");
    let panic_result = std::panic::catch_unwind(|| {
        b4.split_to(15); // This should panic
    });
    assert!(panic_result.is_err()); // Confirm panic occurred
}

