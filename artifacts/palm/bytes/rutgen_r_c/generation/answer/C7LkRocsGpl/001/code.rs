// Answer 0

#[test]
fn test_remaining_non_empty() {
    let bytes = Bytes::from_static(b"Hello, World!");
    assert_eq!(bytes.remaining(), 13);
}

#[test]
fn test_remaining_empty() {
    let bytes = Bytes::new();
    assert_eq!(bytes.remaining(), 0);
}

#[test]
fn test_remaining_large() {
    let long_bytes = Bytes::from_static(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
    assert_eq!(long_bytes.remaining(), 123);
}

