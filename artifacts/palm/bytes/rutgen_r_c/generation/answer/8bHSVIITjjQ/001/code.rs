// Answer 0

#[test]
fn test_bytes_mut_new_empty() {
    let bytes = BytesMut::new();
    assert_eq!(0, bytes.len());
    assert_eq!(0, bytes.capacity());
}

#[test]
fn test_bytes_mut_reserve_with_new() {
    let mut bytes = BytesMut::new();
    bytes.reserve(10);
    assert_eq!(0, bytes.len());
    assert!(bytes.capacity() >= 10);
}

#[test]
fn test_bytes_mut_reserve_for_growth() {
    let mut bytes = BytesMut::new();
    bytes.reserve(5);
    assert!(bytes.capacity() >= 5);
    bytes.reserve(10);
    assert!(bytes.capacity() >= 15);
}

#[test]
fn test_bytes_mut_is_empty() {
    let bytes = BytesMut::new();
    assert!(bytes.is_empty());
}

#[test]
#[should_panic]
fn test_bytes_mut_split_on_empty() {
    let mut bytes = BytesMut::new();
    bytes.split_off(1); // Should panic since 'at' is greater than current length
}

