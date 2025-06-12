// Answer 0

#[test]
fn test_from_shared_invalid_uri_characters() {
    use crate::{Bytes, ErrorKind, InvalidUri};
    
    let invalid_uri_bytes = Bytes::from_static(b"/path?\x7F#fragment"); // contains 0x7F
    let result: Result<_, InvalidUri> = from_shared(invalid_uri_bytes);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ErrorKind::InvalidUriChar.into());
}

#[test]
fn test_from_shared_invalid_query_character() {
    use crate::{Bytes, ErrorKind, InvalidUri};
    
    let invalid_uri_bytes = Bytes::from_static(b"/path?invalid\x7F"); // query contains 0x7F
    let result: Result<_, InvalidUri> = from_shared(invalid_uri_bytes);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ErrorKind::InvalidUriChar.into());
}

#[test]
fn test_from_shared_invalid_path_character() {
    use crate::{Bytes, ErrorKind, InvalidUri};
    
    let invalid_uri_bytes = Bytes::from_static(b"/path\x80#fragment"); // contains 0x80
    let result: Result<_, InvalidUri> = from_shared(invalid_uri_bytes);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), ErrorKind::InvalidUriChar.into());
}

#[test]
fn test_from_shared_missing_query() {
    use crate::{Bytes, ErrorKind, InvalidUri};
    
    let valid_uri_bytes = Bytes::from_static(b"/valid/path#fragment"); // valid path but missing query
    let result: Result<_, InvalidUri> = from_shared(valid_uri_bytes);
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_validate_utf8() {
    use crate::{Bytes, ErrorKind, InvalidUri};
    
    let mixed_uri_bytes = Bytes::from_static(b"/mixed/path?query#fragment"); // valid URI with valid characters
    let result: Result<_, InvalidUri> = from_shared(mixed_uri_bytes);
    assert!(result.is_ok());
}

