// Answer 0

#[test]
fn test_from_iter_empty() {
    let result: BytesMut = BytesMut::from_iter(vec![]);
    assert_eq!(result.len(), 0);
    assert_eq!(result.capacity(), 0);
}

#[test]
fn test_from_iter_non_empty() {
    let result: BytesMut = BytesMut::from_iter(vec![1, 2, 3, 4]);
    assert_eq!(result.len(), 4);
    assert_eq!(result.capacity(), 4); // Assuming Vec will allocate equal capacity.
    assert_eq!(result.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn test_from_iter_large() {
    let result: BytesMut = BytesMut::from_iter((0..100).collect::<Vec<u8>>());
    assert_eq!(result.len(), 100);
    assert!(result.capacity() >= 100); // Capacity may be greater than len.
}

