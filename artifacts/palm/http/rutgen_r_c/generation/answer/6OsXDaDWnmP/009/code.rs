// Answer 0

#[test]
fn test_parse_hdr_empty_data() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

#[test]
fn test_parse_hdr_overflow_data() {
    let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]; // Length greater than SCRATCH_BUF_SIZE
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

#[test]
fn test_parse_hdr_invalid_length() {
    let data: &[u8] = &[0; SCRATCH_BUF_OVERFLOW]; // Length is SCRATCH_BUF_OVERFLOW
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

