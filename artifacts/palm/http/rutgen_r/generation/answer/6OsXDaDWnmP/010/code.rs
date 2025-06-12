// Answer 0

#[test]
fn test_parse_hdr_empty_data() {
    let data: &[u8] = &[];
    let mut buffer: [MaybeUninit<u8>; SCRATCH_BUF_SIZE] = Default::default();
    let table: [u8; 256] = [0; 256]; // Initialize a dummy table

    let result = parse_hdr(data, &mut buffer, &table);
    assert!(result.is_err());
    assert_eq!(result.err(), Some(InvalidHeaderName::new()));
}

