// Answer 0

#[test]
fn test_invalid_uri_fmt_invalid_uri_char() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::InvalidUriChar));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid uri character");
}

#[test]
fn test_invalid_uri_fmt_invalid_scheme() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::InvalidScheme));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid scheme");
}

#[test]
fn test_invalid_uri_fmt_invalid_authority() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::InvalidAuthority));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid authority");
}

#[test]
fn test_invalid_uri_fmt_invalid_port() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::InvalidPort));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid port");
}

#[test]
fn test_invalid_uri_fmt_invalid_format() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::InvalidFormat));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid format");
}

#[test]
fn test_invalid_uri_fmt_scheme_missing() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::SchemeMissing));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "scheme missing");
}

#[test]
fn test_invalid_uri_fmt_authority_missing() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::AuthorityMissing));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "authority missing");
}

#[test]
fn test_invalid_uri_fmt_path_and_query_missing() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::PathAndQueryMissing));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "path missing");
}

#[test]
fn test_invalid_uri_fmt_too_long() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::TooLong));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "uri too long");
}

#[test]
fn test_invalid_uri_fmt_empty() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::Empty));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "empty string");
}

#[test]
fn test_invalid_uri_fmt_scheme_too_long() {
    struct InvalidUriTest(InvalidUri);
    
    let invalid_uri = InvalidUriTest(InvalidUri(ErrorKind::SchemeTooLong));
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_uri.0);
    
    assert!(result.is_ok());
    assert_eq!(output, "scheme too long");
}

