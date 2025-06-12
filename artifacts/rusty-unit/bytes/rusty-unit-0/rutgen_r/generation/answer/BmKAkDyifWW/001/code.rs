// Answer 0

#[test]
fn test_copy_to_bytes_valid_length() {
    use bytes::{BytesMut, Bytes};

    let mut buf = BytesMut::from("Hello, world!");
    let result: Bytes = buf.copy_to_bytes(5);
    assert_eq!(&result[..], b"Hello");
    assert_eq!(buf.len(), 8); // Remaining bytes in buf
}

#[test]
#[should_panic(expected = "not enough bytes in the buffer")]
fn test_copy_to_bytes_too_long() {
    use bytes::{BytesMut};

    let mut buf = BytesMut::from("Hello");
    let _result: Bytes = buf.copy_to_bytes(10); // This should panic
}

#[test]
fn test_copy_to_bytes_zero_length() {
    use bytes::{BytesMut, Bytes};

    let mut buf = BytesMut::from("Test");
    let result: Bytes = buf.copy_to_bytes(0);
    assert_eq!(result.len(), 0);
    assert_eq!(buf.len(), 4); // Remaining bytes are unchanged
}

#[test]
fn test_copy_to_bytes_exact_length() {
    use bytes::{BytesMut, Bytes};

    let mut buf = BytesMut::from("ExactLength");
    let result: Bytes = buf.copy_to_bytes(11);
    assert_eq!(&result[..], b"ExactLength");
    assert_eq!(buf.len(), 0); // All bytes consumed
}

