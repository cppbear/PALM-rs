// Answer 0

#[test]
fn test_from_iter_empty() {
    let bytes_mut: BytesMut = BytesMut::from_iter(vec![]);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.capacity(), 0);
}

#[test]
fn test_from_iter_single() {
    let bytes_mut: BytesMut = BytesMut::from_iter(vec![1]);
    assert_eq!(bytes_mut.len(), 1);
    assert_eq!(bytes_mut.as_slice(), &[1]);
}

#[test]
fn test_from_iter_multiple() {
    let bytes_mut: BytesMut = BytesMut::from_iter(vec![1, 2, 3, 4, 5]);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_from_iter_large() {
    let large_data: Vec<u8> = (0..1000).collect();
    let bytes_mut: BytesMut = BytesMut::from_iter(large_data.iter().cloned());
    assert_eq!(bytes_mut.len(), 1000);
    assert_eq!(bytes_mut.capacity(), 1000);
}

#[should_panic]
fn test_from_iter_panic() {
    let _: BytesMut = BytesMut::from_iter((0..usize::MAX).collect::<Vec<_>>());
}

