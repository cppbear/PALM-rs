// Answer 0

#[test]
fn test_deserialize_valid_utf8_bytes() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"some bytes: \xe5\x00\xe5\"";
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();

    assert_eq!(b'\xe5', bytes[12]);
    assert_eq!(b'\0', bytes[13]);
    assert_eq!(b'\xe5', bytes[14]);
}

#[test]
fn test_deserialize_lone_surrogate() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"lone surrogate: \\uD801\"";
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();
    let expected = b"lone surrogate: \xED\xA0\x81";

    assert_eq!(expected, bytes.as_slice());
}

#[test]
#[should_panic]
fn test_deserialize_invalid_utf8_bytes() {
    use serde_bytes::ByteBuf;
    let json_data = b"\xe5\x00\x80"; // Invalid UTF-8 sequence
    let result: Result<ByteBuf, _> = serde_json::from_slice(json_data);
    assert!(result.is_err());
}

