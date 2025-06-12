// Answer 0

#[test]
fn test_parse_exact_valid_scheme() {
    const MAX_SCHEME_LEN: usize = 10; // Example maximum length, set accordingly
    const SCHEME_CHARS: [u8; 256] = [1; 256]; // Simplified SCHEME_CHARS initialization
    const SCHEME_CHARS_TEST: [u8; 256] = [0; 256]; // Placeholders for actual SCHEME_CHARS
    SCHEME_CHARS_TEST[b'a' as usize] = 1; // Assuming 'a' is valid
    SCHEME_CHARS_TEST[b'b' as usize] = 2; // Assuming 'b' is valid
    SCHEME_CHARS_TEST[b':' as usize] = b':'; // Invalid character
    SCHEME_CHARS_TEST[b'c' as usize] = 3; // Assuming 'c' is valid too

    let input: &[u8] = b"abc"; // meets length constraint and valid characters

    let result = parse_exact(input);

    assert!(result.is_ok());
    match result {
        Ok(scheme) => {
            // Further checks could be made on the scheme if needed
        },
        Err(_) => panic!("Expected Ok but got Err"),
    }
}

#[test]
#[should_panic]
fn test_parse_exact_invalid_character() {
    const SCHEME_CHARS: [u8; 256] = [1; 256]; 
    const SCHEME_CHARS_TEST: [u8; 256] = [0; 256]; 
    SCHEME_CHARS_TEST[b'a' as usize] = 1; // Assuming 'a' is valid
    SCHEME_CHARS_TEST[b'1' as usize] = 0; // Assuming '1' is invalid

    let input: &[u8] = b"1"; // '1' is invalid so should panic

    let _ = parse_exact(input);
}

#[test]
#[should_panic]
fn test_parse_exact_too_long_scheme() {
    const MAX_SCHEME_LEN: usize = 10; // Example maximum length
    let input: &[u8] = b"abcdefghij"; // Exactly MAX_SCHEME_LEN, valid

    let result = parse_exact(input);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_exact_scheme_contains_invalid_byte() {
    const MAX_SCHEME_LEN: usize = 10; 
    let input: &[u8] = b"abc:de"; // Contains a ':', which should trigger an error

    let result = parse_exact(input);

    assert!(result.is_err());
}


