// Answer 0

#[test]
fn test_split_to_must_use() {
    use bytes::Bytes;

    // Test 1: Basic split with valid length
    let mut b1 = Bytes::from("hello world");
    let result1 = b1.split_to(6);
    assert_eq!(result1, Bytes::from("hello "));

    // Test 2: Split with length equal to the length of Bytes
    let mut b2 = Bytes::from("hello");
    let result2 = b2.split_to(5);
    assert_eq!(result2, Bytes::from("hello"));

    // Test 3: Split resulting in empty Bytes
    let mut b3 = Bytes::from("");
    let result3 = b3.split_to(0);
    assert_eq!(result3, Bytes::from(""));

    // Test 4: Attempting to split more than the length of Bytes, should panic
    let mut b4 = Bytes::from("test");
    let result4 = std::panic::catch_unwind(|| {
        b4.split_to(5);
    });
    assert!(result4.is_err());
}

