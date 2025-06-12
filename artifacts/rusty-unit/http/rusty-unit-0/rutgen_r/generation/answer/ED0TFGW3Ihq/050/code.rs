// Answer 0

#[test]
fn test_parse_exact_invalid_scheme_too_long() {
    const MAX_SCHEME_LEN: usize = 10; // Example maximum length
    const SCHEME_CHARS: [u8; 256] = [1; 256]; // All characters are valid except for the chosen ones

    // Simulate the invalid character case
    let input: &[u8] = b"abcdefghij"; // Length is exactly MAX_SCHEME_LEN

    // Making SCHEME_CHARS[b'a'] and others invalid (0)
    SCHEME_CHARS[b'a' as usize] = 0;
    SCHEME_CHARS[b'b' as usize] = 0;
    SCHEME_CHARS[b'c' as usize] = 0;
    SCHEME_CHARS[b'd' as usize] = 0;
    SCHEME_CHARS[b'e' as usize] = 0;
    SCHEME_CHARS[b'f' as usize] = 0;
    SCHEME_CHARS[b'g' as usize] = 0;
    SCHEME_CHARS[b'h' as usize] = 0;
    SCHEME_CHARS[b'i' as usize] = 0;
    SCHEME_CHARS[b'j' as usize] = 0;

    let result = parse_exact(input);
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind(), &ErrorKind::InvalidScheme);
    }
}

