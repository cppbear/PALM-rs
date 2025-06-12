// Answer 0

#[test]
fn test_scheme_http_as_str() {
    let scheme: Scheme = Scheme::HTTP;
    scheme.as_str();
}

#[test]
fn test_scheme_https_as_str() {
    let scheme: Scheme = Scheme::HTTPS;
    scheme.as_str();
}

#[test]
fn test_scheme_empty_as_str() {
    let scheme: Scheme = Scheme::empty();
    scheme.as_str();
}

