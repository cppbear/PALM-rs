// Answer 0

#[test]
fn test_parse_hdr_empty_length() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    assert!(parse_hdr(data, &mut buffer, &HEADER_CHARS).is_err());
}

#[test]
fn test_parse_hdr_within_buffer_size() {
    let data: &[u8] = b"valid-header";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    assert!(parse_hdr(data, &mut buffer, &HEADER_CHARS).is_ok());
}

#[test]
fn test_parse_hdr_overflow_length() {
    let data: &[u8] = &[b'a'; SCRATCH_BUF_OVERFLOW];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());
    if let Ok(hdr_name) = result {
        assert_eq!(hdr_name.inner, Repr::Custom(MaybeLower { buf: data, lower: false }));
    }
}

#[test]
fn test_parse_hdr_exceed_max_length() {
    let data: &[u8] = &[b'a'; super::MAX_HEADER_NAME_LEN + 1];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    assert!(parse_hdr(data, &mut buffer, &HEADER_CHARS).is_err());
}

