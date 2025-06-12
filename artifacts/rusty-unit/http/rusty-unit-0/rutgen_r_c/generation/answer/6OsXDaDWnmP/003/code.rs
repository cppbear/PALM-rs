// Answer 0

fn test_parse_hdr_empty() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

fn test_parse_hdr_valid_custom() {
    let data: &[u8] = b"custom-header";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());
    if let Ok(hdr_name) = result {
        // Check custom name characteristics
        assert_eq!(hdr_name.inner, Repr::Custom(MaybeLower { buf: data, lower: true }));
    }
}

fn test_parse_hdr_valid_standard_header() {
    let data: &[u8] = b"accept";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());
    // Check if it converts to standard header
    if let Ok(hdr_name) = result {
        match hdr_name.inner {
            Repr::Standard(_) => {}
            _ => panic!("Expected standard header, but got something else"),
        }
    }
}

fn test_parse_hdr_invalid_with_null_character() {
    let data: &[u8] = b"invalid\0header";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

fn test_parse_hdr_overflow() {
    let data: &[u8] = &[1; SCRATCH_BUF_OVERFLOW]; // Generate a buffer larger than allowed
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());
} 

fn test_parse_hdr_custom_large_length() {
    let data = b"more-than-64-characters-header-name";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());
    if let Ok(hdr_name) = result {
        assert_eq!(hdr_name.inner, Repr::Custom(MaybeLower { buf: data, lower: false }));
    }
}

