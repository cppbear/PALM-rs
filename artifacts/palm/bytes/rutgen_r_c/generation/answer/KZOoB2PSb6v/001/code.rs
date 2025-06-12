// Answer 0

#[test]
fn test_freeze_with_kind_vec() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.extend_from_slice(&b"hello world"[..]);
    let frozen_bytes = bytes_mut.freeze();
    
    assert_eq!(&frozen_bytes.slice(..).as_slice(), b"hello world");

    let second_frozen_bytes = frozen_bytes.clone();
    assert_eq!(&second_frozen_bytes.slice(..).as_slice(), b"hello world");
}

#[test]
fn test_freeze_empty_bytesmut() {
    let bytes_mut = BytesMut::new();
    let frozen_bytes = bytes_mut.freeze();
    
    assert!(frozen_bytes.is_empty());
}

#[test]
fn test_freeze_max_capacity_bytesmut() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    let input_data = vec![0u8; usize::MAX];
    bytes_mut.extend_from_slice(&input_data);
    let frozen_bytes = bytes_mut.freeze();
    
    assert_eq!(frozen_bytes.len(), usize::MAX);
}

