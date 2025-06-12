// Answer 0

#[test]
fn test_invalid_uri_too_long() {
    use crate::InvalidUri;
    
    let error_kind = ErrorKind::TooLong;
    let invalid_uri = InvalidUri(error_kind);
    let _result = invalid_uri.s();
}

