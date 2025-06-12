// Answer 0

#[test]
fn test_http_scheme_as_str() {
    let scheme = Scheme::HTTP;
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_https_scheme_as_str() {
    let scheme = Scheme::HTTPS;
    assert_eq!(scheme.as_str(), "https");
}

#[test]
fn test_empty_scheme_as_str() {
    let scheme = Scheme::empty();
    // Since we are calling unreachable in as_str for None,
    // we expect a panic to occur here.
    let result = std::panic::catch_unwind(|| {
        scheme.as_str();
    });
    assert!(result.is_err());
}

