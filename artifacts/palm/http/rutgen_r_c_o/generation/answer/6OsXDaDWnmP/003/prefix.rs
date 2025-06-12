// Answer 0

#[test]
fn test_parse_hdr_empty() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_valid_range() {
    let data: &[u8] = b"valid-header";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_zero_in_header() {
    let data: &[u8] = b"header\0test";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_not_standard_header() {
    let data: &[u8] = b"not-standard-header";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_maximum_length() {
    let data: &[u8] = b"header-maximum-length-which-is-up-to-64-bytes-length";
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

#[test]
fn test_parse_hdr_overflow_length() {
    let data: &[u8] = &vec![b'a'; 65];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let result = parse_hdr(data, &mut buffer, &HEADER_CHARS);
}

