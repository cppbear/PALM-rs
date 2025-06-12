// Answer 0

#[test]
fn test_decode_hex_escape_valid_escape() {
    // Test valid hex escape sequence
    let mut slice_read = SliceRead {
        slice: b"\\uABCDxyz",
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert_eq!(result, Ok(0xABCD)); // Expected to return the decoded value
    assert_eq!(slice_read.index, 4); // Index should now be at 4
}

#[test]
fn test_decode_hex_escape_invalid_hex() {
    // Test invalid hex escape sequence
    let mut slice_read = SliceRead {
        slice: b"\\uXYZDxyz",
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_err()); // Expected to return an error
}

#[test]
fn test_decode_hex_escape_eof() {
    // Test end of hex escape
    let mut slice_read = SliceRead {
        slice: b"\\u123",
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_err()); // Expected to return an EOF error
    assert_eq!(slice_read.index, 4); // Index should point to the end of slice
}

#[test]
fn test_decode_hex_escape_empty_slice() {
    // Test when slice is empty
    let mut slice_read = SliceRead {
        slice: b"",
        index: 0,
    };

    let result = slice_read.decode_hex_escape();
    assert!(result.is_err()); // Expected to return an EOF error
}

