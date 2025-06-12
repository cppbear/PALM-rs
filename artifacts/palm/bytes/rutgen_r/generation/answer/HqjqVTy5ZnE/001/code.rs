// Answer 0

#[test]
fn test_split_to_must_use() {
    use bytes::BytesMut;

    // Create a BytesMut instance with initial data
    let mut b1 = BytesMut::from("hello world");
    
    // Test proper usage of split_to
    let split_part = b1.split_to(6);
    
    // Check the values of the original and the split part
    assert_eq!(&*b1, " world");
    assert_eq!(&*split_part, "hello ");
}

#[test]
#[should_panic]
fn test_split_to_panic_too_large() {
    use bytes::BytesMut;

    // Create a BytesMut instance with initial data
    let mut b1 = BytesMut::from("hello world");
    
    // Attempt to split more bytes than available, expecting a panic
    let _ = b1.split_to(15);
}

