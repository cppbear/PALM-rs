// Answer 0

#[test]
fn test_chunk_non_empty() {
    let bytes = Bytes::from_static(b"hello");
    let chunk = bytes.chunk();
    assert_eq!(chunk, b"hello");
}

#[test]
fn test_chunk_empty() {
    let bytes = Bytes::new();
    let chunk = bytes.chunk();
    assert_eq!(chunk, b"");
}

