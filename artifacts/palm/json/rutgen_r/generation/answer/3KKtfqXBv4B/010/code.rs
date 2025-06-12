// Answer 0

#[test]
fn test_deserialize_bytes_valid_utf8() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"valid utf-8 bytes: \xe2\x9c\x94\""; // JSON string
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();
    let expected = b"valid utf-8 bytes: \xE2\x9C\x94"; // Unicode checkmark
    assert_eq!(expected, bytes.as_slice());
}

#[test]
fn test_deserialize_bytes_invalid_utf8() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"bytes with invalid utf-8: \xe5\x00\xe5\""; // JSON string
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();
    assert_eq!(b'\xe5', bytes[25]);
    assert_eq!(b'\0', bytes[26]);
    assert_eq!(b'\xe5', bytes[27]);
}

#[test]
fn test_deserialize_bytes_with_lone_surrogate() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"lone surrogate: \\uD801\""; // JSON string
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();
    let expected = b"lone surrogate: \xED\xA0\x81";
    assert_eq!(expected, bytes.as_slice());
}

#[test]
fn test_deserialize_bytes_empty_string() {
    use serde_bytes::ByteBuf;
    let json_data = b"\"\""; // Empty JSON string
    let bytes: ByteBuf = serde_json::from_slice(json_data).unwrap();
    assert_eq!(b"", bytes.as_slice());
}

