// Answer 0

#[test]
fn test_parse_scheme_too_long() {
    const MAX_SCHEME_LEN: usize = 10; // Adjust as per actual MAX_SCHEME_LEN
    const SCHEME_CHARS: [u8; 256] = [0; 256]; // Initialize SCHEME_CHARS as needed
    
    // Fill in SCHEME_CHARS accordingly; for this test, we focus on 'b':' as valid
    SCHEME_CHARS[58] = b':' as u8; // Assuming ':' is valid

    let input: &[u8] = b"abcdefghij:////"; // Length = 14, i = 10 (valid) but i > MAX_SCHEME_LEN

    let result = parse(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorKind::SchemeTooLong.into());
}

