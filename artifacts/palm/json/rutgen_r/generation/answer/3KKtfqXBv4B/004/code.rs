// Answer 0

#[test]
fn test_deserialize_bytes_valid_input() {
    use serde_bytes::ByteBuf;

    let json_data = b"\"some bytes: \xe5\x00\xe5\"";
    let bytes: Result<ByteBuf, serde_json::Error> = serde_json::from_slice(json_data);

    assert!(bytes.is_ok());
    let bytes = bytes.unwrap();
    assert_eq!(b'\xe5', bytes[12]);
    assert_eq!(b'\0', bytes[13]);
    assert_eq!(b'\xe5', bytes[14]);
}

#[test]
fn test_deserialize_bytes_unpaired_surrogate() {
    use serde_bytes::ByteBuf;

    let json_data = b"\"lone surrogate: \\uD801\"";
    let bytes: Result<ByteBuf, serde_json::Error> = serde_json::from_slice(json_data);
    
    assert!(bytes.is_err());
}

#[test]
fn test_deserialize_bytes_invalid_utf8() {
    use serde_bytes::ByteBuf;

    let json_data = b"\"invalid utf8: \xe7\x80\""; // Last byte is invalid
    let bytes: Result<ByteBuf, serde_json::Error> = serde_json::from_slice(json_data);

    assert!(bytes.is_err());
}

#[test]
fn test_deserialize_bytes_empty_array() {
    use serde_bytes::ByteBuf;

    let json_data = b"[]";
    let bytes: Result<ByteBuf, serde_json::Error> = serde_json::from_slice(json_data);

    assert!(bytes.is_err());
}

#[test]
fn test_deserialize_bytes_no_value() {
    use serde_bytes::ByteBuf;

    let json_data = b"\"\""; // valid but empty string
    let bytes: Result<ByteBuf, serde_json::Error> = serde_json::from_slice(json_data);

    assert!(bytes.is_ok());
    assert_eq!(bytes.unwrap().as_slice(), b"");
}

