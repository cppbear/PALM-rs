// Answer 0

#[test]
fn test_inc_start_valid() {
    let mut bytes = Bytes::from_static(b"Hello, World!");
    let initial_len = bytes.len();
    unsafe { bytes.inc_start(7) };
    assert_eq!(bytes.len(), initial_len - 7);
    assert_eq!(bytes.as_slice(), b"World!");
}

#[test]
#[should_panic(expected = "internal: inc_start out of bounds")]
fn test_inc_start_too_large() {
    let mut bytes = Bytes::from_static(b"Hello, World!");
    unsafe { bytes.inc_start(20) }; // This should panic
}

#[test]
fn test_inc_start_zero() {
    let mut bytes = Bytes::from_static(b"Hello, World!");
    let initial_len = bytes.len();
    unsafe { bytes.inc_start(0) };
    assert_eq!(bytes.len(), initial_len);
    assert_eq!(bytes.as_slice(), b"Hello, World!");
}

#[test]
fn test_inc_start_equal_length() {
    let mut bytes = Bytes::from_static(b"Hello, World!");
    unsafe { bytes.inc_start(bytes.len()) };
    assert_eq!(bytes.len(), 0);
    assert_eq!(bytes.as_slice(), b""); // Expecting an empty slice after incrementing by its length
}

