// Answer 0

#[test]
fn test_parse_hdr_empty_input() {
    let mut scratch: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(&[], &mut scratch, &HEADER_CHARS);
    assert!(result.is_err());
}

#[test]
fn test_parse_hdr_valid_standard_header() {
    let mut scratch: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let header_bytes = b"accept";
    let result = parse_hdr(header_bytes, &mut scratch, &HEADER_CHARS);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hdr_invalid_character() {
    let mut scratch: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let header_bytes = b"invalid\x00header";
    let result = parse_hdr(header_bytes, &mut scratch, &HEADER_CHARS);
    assert!(result.is_err());
}

#[test]
fn test_parse_hdr_too_long_header() {
    let mut scratch: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let header_bytes = vec![b'a'; SCRATCH_BUF_OVERFLOW].as_slice();
    let result = parse_hdr(header_bytes, &mut scratch, &HEADER_CHARS);
    assert!(result.is_ok());
}

#[test]
fn test_parse_hdr_exceeds_max_length() {
    let mut scratch: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let header_bytes = vec![b'a'; super::MAX_HEADER_NAME_LEN + 1].as_slice();
    let result = parse_hdr(header_bytes, &mut scratch, &HEADER_CHARS);
    assert!(result.is_err());
}

