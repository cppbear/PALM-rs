// Answer 0

#[test]
fn test_decode_hex_escape_eof_while_parsing_string() {
    let mut slice_read = SliceRead {
        slice: b"" as &[u8],
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::EofWhileParsingString);
}

#[test]
fn test_decode_hex_escape_invalid_escape() {
    let mut slice_read = SliceRead {
        slice: b"\x00\x01\x02", // Not enough bytes for a valid hex escape
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().code(), ErrorCode::EofWhileParsingString);
}

#[test]
fn test_decode_hex_escape_success() {
    let mut slice_read = SliceRead {
        slice: b"\x41\x42\x43\x44", // Valid hex escape for 'A'
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0x4142u16); // This checks if it returns the correct value for 'AB'
}

