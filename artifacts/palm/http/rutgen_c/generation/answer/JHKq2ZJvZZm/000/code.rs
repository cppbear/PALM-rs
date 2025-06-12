// Answer 0

#[test]
fn test_try_from_valid_http_scheme() {
    let input = b"http";
    let result = Scheme::try_from(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme.inner {
            Scheme2::Standard(Protocol::Http) => {}
            _ => panic!("Expected Standard(Http) variant"),
        }
    }
}

#[test]
fn test_try_from_valid_https_scheme() {
    let input = b"https";
    let result = Scheme::try_from(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme.inner {
            Scheme2::Standard(Protocol::Https) => {}
            _ => panic!("Expected Standard(Https) variant"),
        }
    }
}

#[test]
fn test_try_from_invalid_scheme_too_long() {
    let input = b"this_is_a_very_long_scheme_name_exceeding_the_limit";
    let result = Scheme::try_from(input);
    assert!(result.is_err());
    if let Err(InvalidUri(ErrorKind::SchemeTooLong)) = result {
        // Expected error
    } else {
        panic!("Expected ErrorKind::SchemeTooLong");
    }
}

#[test]
fn test_try_from_invalid_scheme_char() {
    let input = b"ht!tp";
    let result = Scheme::try_from(input);
    assert!(result.is_err());
    if let Err(InvalidUri(ErrorKind::InvalidScheme)) = result {
        // Expected error
    } else {
        panic!("Expected ErrorKind::InvalidScheme");
    }
}

#[test]
fn test_try_from_other_scheme() {
    let input = b"custom_scheme";
    let result = Scheme::try_from(input);
    assert!(result.is_ok());
    if let Ok(scheme) = result {
        match scheme.inner {
            Scheme2::Other(_) => {}
            _ => panic!("Expected Other variant"),
        }
    }
}

#[test]
#[should_panic]
fn test_try_from_invalid_utf8_bytes() {
    let input = [0xFF]; // Invalid UTF-8 sequence
    let _ = Scheme::try_from(&input);
}

