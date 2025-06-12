// Answer 0

#[test]
fn test_len_empty() {
    let b = bytes::Bytes::from(&b""[..]);
    assert_eq!(b.len(), 0);
}

#[test]
fn test_len_single_byte() {
    let b = bytes::Bytes::from(&b"a"[..]);
    assert_eq!(b.len(), 1);
}

#[test]
fn test_len_multiple_bytes() {
    let b = bytes::Bytes::from(&b"hello"[..]);
    assert_eq!(b.len(), 5);
}

#[test]
fn test_len_long_byte_array() {
    let b = bytes::Bytes::from(&b"example data for testing purposes"[..]);
    assert_eq!(b.len(), 30);
}

#[test]
fn test_len_non_ascii_bytes() {
    let b = bytes::Bytes::from(&b"\xE2\x9C\x94"[..]); // Check for 3 bytes that represent a check mark
    assert_eq!(b.len(), 3);
}

