// Answer 0

#[test]
fn test_chunk_non_empty() {
    let bytes = Bytes::from_static(b"Hello, world!");
    let result = bytes.chunk();
    assert_eq!(result, b"Hello, world!");
}

#[test]
fn test_chunk_empty() {
    let bytes = Bytes::from_static(b"");
    let result = bytes.chunk();
    assert_eq!(result, b"");
}

#[test]
#[should_panic(expected = "out of range")]
fn test_chunk_invalid_range() {
    let bytes = Bytes::from_static(b"Test");
    let _ = bytes.slice(5..10); // This should panic due to out of bounds
}

#[test]
fn test_chunk_after_split_to() {
    let mut bytes = Bytes::from_static(b"Hello");
    let split = bytes.split_to(3);
    let result = bytes.chunk();
    assert_eq!(result, b"lo");
    assert_eq!(split.chunk(), b"Hel");
}

#[test]
fn test_chunk_after_split_off() {
    let mut bytes = Bytes::from_static(b"Hello");
    let split = bytes.split_off(3);
    let result = bytes.chunk();
    assert_eq!(result, b"Hel");
    assert_eq!(split.chunk(), b"lo");
}

