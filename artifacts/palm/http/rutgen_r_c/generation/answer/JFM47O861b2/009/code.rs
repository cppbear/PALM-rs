// Answer 0

#[test]
fn test_from_shared_empty_uri() {
    use bytes::Bytes;
    use crate::uri::{Uri, InvalidUri};

    let result: Result<Uri, InvalidUri> = Uri::from_shared(Bytes::from_static(&[]));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::Empty);
}

#[test]
fn test_from_shared_single_slash() {
    use bytes::Bytes;
    use crate::uri::{Uri, InvalidUri};

    let result: Result<Uri, InvalidUri> = Uri::from_shared(Bytes::from_static(b"/"));
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path_and_query().as_str(), "/");
}

#[test]
fn test_from_shared_single_star() {
    use bytes::Bytes;
    use crate::uri::{Uri, InvalidUri};

    let result: Result<Uri, InvalidUri> = Uri::from_shared(Bytes::from_static(b"*"));
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert!(uri.scheme().is_none());
    assert!(uri.authority().is_none());
    assert_eq!(uri.path_and_query().as_str(), "*");
}

#[test]
fn test_from_shared_max_len() {
    use bytes::Bytes;
    use crate::uri::{Uri, InvalidUri};

    let max_len_bytes = vec![b'a'; MAX_LEN];
    let result: Result<Uri, InvalidUri> = Uri::from_shared(Bytes::from_static(&max_len_bytes));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().0, ErrorKind::TooLong);
}

