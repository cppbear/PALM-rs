// Answer 0

#[test]
fn test_split_to_must_use() {
    use bytes::BytesMut;

    let mut b1 = BytesMut::from("hello world");
    let result = b1.split_to(6);
    
    // Verify the result
    assert_eq!(result.into_vec(), b"hello ".to_vec());
    assert_eq!(b1.into_vec(), b"world".to_vec());
}

#[test]
#[should_panic]
fn test_split_to_must_use_panic() {
    use bytes::BytesMut;

    let mut b1 = BytesMut::from("hello world");
    // The following line should panic due to unused result
    let _ = b1.split_to(6);
}

