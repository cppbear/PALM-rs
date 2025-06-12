// Answer 0

#[test]
fn test_parse_hdr_empty_data() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_too_long_data() {
    let data: &[u8] = &[0; SCRATCH_BUF_OVERFLOW + 1];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

