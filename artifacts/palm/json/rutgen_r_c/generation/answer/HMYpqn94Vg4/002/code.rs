// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    let mut slice_read = SliceRead {
        slice: &[0x31, 0x32, 0x33, 0x34], // '1234' in hex
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    
    assert_eq!(result, Ok(0x1234)); // Successfully decode '1234' 
}

#[test]
fn test_decode_hex_escape_eof() {
    let mut slice_read = SliceRead {
        slice: &[0x31, 0x32, 0x33], // Only '123' present, not enough bytes for a hex escape
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    
    assert!(result.is_err()); // Should be an error due to insufficient bytes
}

#[test]
fn test_decode_hex_escape_invalid() {
    let mut slice_read = SliceRead {
        slice: &[0x67, 0x68, 0x69, 0x6A], // 'ghij' which doesn't represent valid hex digits
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    
    assert!(result.is_err()); // Should be an error due to invalid escape sequence
}

