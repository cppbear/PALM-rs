// Answer 0

#[test]
fn test_chunk_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    let chunk = bytes_mut.chunk();
    assert_eq!(chunk, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    let bytes_mut = BytesMut::new();
    
    let chunk = bytes_mut.chunk();
    assert_eq!(chunk, &[]);
}

#[test]
fn test_chunk_after_resize() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.resize(3, 0);
    
    let chunk = bytes_mut.chunk();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_after_clear() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.clear();
    
    let chunk = bytes_mut.chunk();
    assert_eq!(chunk, &[]);
}

