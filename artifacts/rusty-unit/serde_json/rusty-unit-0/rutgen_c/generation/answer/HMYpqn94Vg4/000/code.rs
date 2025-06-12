// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    let mut slice_read = SliceRead {
        slice: &[b'0', b'1', b'0', b'1'],
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    assert_eq!(result.unwrap(), 0x0101);
    assert_eq!(slice_read.index, 4);
}

#[test]
fn test_decode_hex_escape_invalid_escape() {
    let mut slice_read = SliceRead {
        slice: &[b'0', b'1', b'G', b'1'], // 'G' is invalid in hex
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    assert!(result.is_err());
    assert_eq!(slice_read.index, 4); // even though invalid, the index is incremented
}

#[test]
fn test_decode_hex_escape_eof() {
    let mut slice_read = SliceRead {
        slice: &[b'0', b'1'], // not enough bytes to read 4 hex digits
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    assert!(result.is_err());
    assert_eq!(slice_read.index, 2); // index should be set to the end
}

#[test]
fn test_decode_hex_escape_edge_case() {
    let mut slice_read = SliceRead {
        slice: &[b'0', b'1', b'0', b'1', b'A'], // valid hex followed by extra byte
        index: 0,
    };
    let result = slice_read.decode_hex_escape();
    assert_eq!(result.unwrap(), 0x0101);
    assert_eq!(slice_read.index, 4);
}

