// Answer 0

#[test]
fn test_scheme_empty() {
    let scheme = Scheme::empty();
    assert_eq!(scheme.inner, Scheme2::None);
}

#[test]
fn test_scheme_http_constant() {
    let scheme = Scheme::HTTP;
    assert_eq!(scheme.inner, Scheme2::Standard(Protocol::Http));
}

#[test]
fn test_scheme_https_constant() {
    let scheme = Scheme::HTTPS;
    assert_eq!(scheme.inner, Scheme2::Standard(Protocol::Https));
}

