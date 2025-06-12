// Answer 0

#[test]
fn test_as_str_http() {
    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    };
    scheme.as_str();
}

#[test]
fn test_as_str_https() {
    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Https),
    };
    scheme.as_str();
}

#[test]
#[should_panic]
fn test_as_str_none() {
    let scheme = Scheme::empty();
    scheme.as_str();
}

