// Answer 0

#[test]
#[should_panic]
fn test_from_utf8_unchecked_invalid_utf8_debug() {
    use http::byte_str::{from_utf8_unchecked, ByteStr};
    use bytes::Bytes;

    let invalid_utf8 = Bytes::from(&[0, 159, 146, 150][..]); // These bytes do not represent valid UTF-8
    unsafe {
        from_utf8_unchecked(invalid_utf8);
    }
}

#[test]
fn test_from_utf8_unchecked_valid_utf8() {
    use http::byte_str::{from_utf8_unchecked, ByteStr};
    use bytes::Bytes;

    let valid_utf8 = Bytes::from("valid utf8");
    let result = unsafe { from_utf8_unchecked(valid_utf8) };
    assert_eq!(result.to_string(), "valid utf8");
}

