// Answer 0

#[test]
fn test_from_shared_too_long() {
    let long_input = Bytes::from(vec![0; http::MAX_LEN + 1]);
    let result = http::Uri::from_shared(long_input);
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.kind(), http::ErrorKind::TooLong);
    }
}

