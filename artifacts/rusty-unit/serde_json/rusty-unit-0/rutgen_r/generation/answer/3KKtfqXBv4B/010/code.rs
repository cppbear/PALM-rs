// Answer 0

#[test]
fn test_successful_deserialize_bytes() {
    use serde_bytes::ByteBuf;
    use serde_json;

    let json_data = b"\"some bytes: \xe5\x00\xe5\"";
    let bytes: ByteBuf = serde_json::from_slice(json_data).expect("Deserialization should succeed");

    assert_eq!(b'\xe5', bytes[12]);
    assert_eq!(b'\0', bytes[13]);
    assert_eq!(b'\xe5', bytes[14]);
}

#[test]
fn test_successful_deserialize_bytes_with_lone_surrogate() {
    use serde_bytes::ByteBuf;
    use serde_json;

    let json_data = b"\"lone surrogate: \\uD801\"";
    let bytes: ByteBuf = serde_json::from_slice(json_data).expect("Deserialization should succeed");
    let expected = b"lone surrogate: \xED\xA0\x81";

    assert_eq!(expected, bytes.as_slice());
}

#[test]
fn test_error_on_invalid_utf8() {
    use serde_bytes::ByteBuf;
    use serde_json;

    let json_data = b"\"invalid utf8: \xe5\x80\""; // Truncated surrogate
    let result: Result<ByteBuf, _> = serde_json::from_slice(json_data);
    
    assert!(result.is_err());
}

