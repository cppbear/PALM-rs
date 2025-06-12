// Answer 0

#[test]
fn test_parse_http() {
    let input: &[u8] = b"http://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme {
            Scheme2::Other(index) => assert_eq!(index, 7),
            _ => panic!("Expected Scheme2::Other(7)"),
        }
    }
}

#[test]
fn test_parse_https() {
    let input: &[u8] = b"https://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme {
            Scheme2::Other(index) => assert_eq!(index, 8),
            _ => panic!("Expected Scheme2::Other(8)"),
        }
    }
}

#[test]
fn test_parse_custom_scheme() {
    let input: &[u8] = b"mycustom://example.com";
    let result = parse(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme {
            Scheme2::Other(index) => assert_eq!(index, 10), // Assuming 'mycustom' has length 8
            _ => panic!("Expected Scheme2::Other(10)"),
        }
    }
}

#[test]
fn test_parse_invalid_scheme_too_long() {
    let input: &[u8] = b"thisschemeiswaytoolong://example.com";
    let result = parse(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), ErrorKind::SchemeTooLong);
    }
}

#[test]
fn test_parse_invalid_scheme_no_double_slash() {
    let input: &[u8] = b"invalidscheme:example.com";
    let result = parse(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), ErrorKind::InvalidUri);
    }
}

