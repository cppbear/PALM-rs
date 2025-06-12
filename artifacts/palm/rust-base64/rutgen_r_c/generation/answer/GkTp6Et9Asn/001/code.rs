// Answer 0

#[test]
fn test_decode_valid() {
    let input = "SGVsbG8gV29ybGQ="; // "Hello World" in Base64
    let result = decode(input).unwrap();
    assert_eq!(result, b"Hello World");
}

#[test]
fn test_decode_invalid_byte() {
    let input = "SGVsbG8gV29ybGQ@"; // Invalid character '@'
    match decode(input) {
        Err(DecodeError::InvalidByte(pos, byte)) => {
            assert_eq!(pos, 16); // position of '@'
            assert_eq!(byte, b'@');
        },
        _ => panic!("Expected InvalidByte error"),
    }
}

#[test]
fn test_decode_invalid_length() {
    let input = "SGVsbG8="; // Length is invalid for padding
    match decode(input) {
        Err(DecodeError::InvalidLength(len)) => {
            assert_eq!(len, 1); // Only 1 valid base64 character
        },
        _ => panic!("Expected InvalidLength error"),
    }
}

#[test]
fn test_decode_invalid_last_symbol() {
    let input = "SGVsbG8gV29ybGQ===="; // Too much padding
    match decode(input) {
        Err(DecodeError::InvalidLastSymbol(pos, byte)) => {
            assert_eq!(pos, 23); // Position of the invalid last '='
            assert_eq!(byte, b'=');
        },
        _ => panic!("Expected InvalidLastSymbol error"),
    }
}

#[test]
fn test_decode_invalid_padding() {
    let input = "SGVsbG8gV29ybGQ=="; // Valid input but assuming the decoder requires no padding
    // It would depend upon the configuration of the Standard engine, but assuming it does not allow padding,
    // This will call for a panic due to invalid padding.
    match decode(input) {
        Err(DecodeError::InvalidPadding) => {},
        _ => panic!("Expected InvalidPadding error"),
    }
}

