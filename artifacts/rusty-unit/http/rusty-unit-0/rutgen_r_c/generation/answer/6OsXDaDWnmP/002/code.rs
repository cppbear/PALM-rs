// Answer 0

fn test_parse_hdr_empty_data() {
    let data: &[u8] = &[];
    let mut b: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut b, &HEADER_CHARS);
    assert_eq!(result, Err(InvalidHeaderName::new()));
}

fn test_parse_hdr_valid_data_too_long() {
    let data: &[u8] = b"example_header";
    let mut b: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut b, &HEADER_CHARS);
    // This case will return Ok since the length is greater than SCRATCH_BUF_SIZE and not an error condition.
    assert!(result.is_ok());
}

fn test_parse_hdr_valid_data_with_null_byte() {
    let data: &[u8] = b"example\0header";
    let mut b: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut b, &HEADER_CHARS);
    assert_eq!(result, Err(InvalidHeaderName::new()));
}

fn test_parse_hdr_standard_header_not_found() {
    let data: &[u8] = b"unknown_header";
    let mut b: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let result = parse_hdr(data, &mut b, &HEADER_CHARS);
    // This is a case where the header does not match any of the standard headers.
    assert!(result.is_ok());
}

