// Answer 0

#[test]
fn test_parse_exact_valid_other_scheme() {
    let input: &[u8] = b"validscheme"; // A scheme that is neither "http" nor "https"
    
    let result = Scheme2::<()>::parse_exact(input);
    
    assert!(result.is_ok());
    match result {
        Ok(scheme) => match scheme {
            Scheme2::Other(()) => (), // we expect this variant
            _ => panic!("Expected Scheme2::Other(()), found a different variant"),
        },
        _ => panic!("Expected Ok, found an error"),
    }
}

#[test]
fn test_parse_exact_max_length() {
    let input: &[u8] = b"validschemevalidschemevalidschemevalidscheme"; // Length == MAX_SCHEME_LEN
    
    let result = Scheme2::<()>::parse_exact(input);
    
    assert!(result.is_ok());
    match result {
        Ok(scheme) => match scheme {
            Scheme2::Other(()) => (), // we expect this variant
            _ => panic!("Expected Scheme2::Other(()), found a different variant"),
        },
        _ => panic!("Expected Ok, found an error"),
    }
}

