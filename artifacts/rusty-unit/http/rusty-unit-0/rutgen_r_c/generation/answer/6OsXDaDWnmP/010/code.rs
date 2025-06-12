// Answer 0

#[test]
fn test_parse_hdr_empty_input() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
    let table = &HEADER_CHARS;

    let result = parse_hdr(data, &mut buffer, table);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(std::mem::size_of::<InvalidHeaderName>() > 0);
    }
}

