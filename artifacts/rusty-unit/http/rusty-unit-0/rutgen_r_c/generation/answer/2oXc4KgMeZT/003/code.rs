// Answer 0

#[test]
fn test_as_str_http() {
    struct Protocol;
    impl Protocol {
        const Http: Self = Self;
    }

    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Http),
    };
    assert_eq!(scheme.as_str(), "http");
}

#[test]
fn test_as_str_https() {
    struct Protocol;
    impl Protocol {
        const Https: Self = Self;
    }

    let scheme = Scheme {
        inner: Scheme2::Standard(Protocol::Https),
    };
    assert_eq!(scheme.as_str(), "https");
}

#[test]
#[should_panic]
fn test_as_str_none() {
    let scheme = Scheme::empty();
    scheme.as_str(); // This should panic
}

