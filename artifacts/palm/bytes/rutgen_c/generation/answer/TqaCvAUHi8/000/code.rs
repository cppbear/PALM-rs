// Answer 0

#[test]
fn test_chunk_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    assert_eq!(bytes_mut.chunk(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_empty() {
    let bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.chunk(), &[]);
}

#[test]
fn test_chunk_after_advance() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.extend_from_slice(&[10, 20, 30, 40, 50]);
    bytes_mut.advance(2);
    assert_eq!(bytes_mut.chunk(), &[30, 40, 50]);
}

#[test]
fn test_chunk_after_truncate() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.extend_from_slice(&[100, 200, 300, 400, 500]);
    bytes_mut.truncate(3);
    assert_eq!(bytes_mut.chunk(), &[100, 200, 300]);
}

