// Answer 0

fn test_parse_hdr_empty() {
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let input: &[u8] = &[];
    let result = parse_hdr(input, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

fn test_parse_hdr_valid_standard_header() {
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let input: &[u8] = b"accept"; // corresponds to the standard header
    let result = parse_hdr(input, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());

    if let Ok(header_name) = result {
        assert_eq!(header_name.inner, Repr::Standard(StandardHeader::Accept));
    }
}

fn test_parse_hdr_invalid_with_null_byte() {
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let input: &[u8] = b"accept\0extra";
    let result = parse_hdr(input, &mut buffer, &HEADER_CHARS);
    assert!(result.is_err());
}

fn test_parse_hdr_long_valid_standard_header() {
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let input: &[u8] = b"accept-encoding"; // another valid standard header
    let result = parse_hdr(input, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());

    if let Ok(header_name) = result {
        assert_eq!(header_name.inner, Repr::Standard(StandardHeader::AcceptEncoding));
    }
}

fn test_parse_hdr_custom_header() {
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let input: &[u8] = b"custom-header"; 
    let result = parse_hdr(input, &mut buffer, &HEADER_CHARS);
    assert!(result.is_ok());

    if let Ok(header_name) = result {
        assert_eq!(header_name.inner, Repr::Custom(MaybeLower { buf: input, lower: true }));
    }
}

